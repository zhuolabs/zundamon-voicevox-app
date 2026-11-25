use std::path::{Path, PathBuf};
use std::sync::Mutex;
use anyhow::{Context as _, anyhow};
use typed_floats::tf32::PositiveFinite;
use voicevox_core::blocking::{Onnxruntime, OpenJtalk, Synthesizer, VoiceModelFile};
use voicevox_core::{StyleId, StyleType};

#[derive(Clone, Debug)]
pub struct StyleInfo {
    pub name: String,
    pub id: u32,
}

#[derive(Clone, Debug)]
pub struct CharacterInfo {
    pub name: String,
    pub styles: Vec<StyleInfo>,
}

/// 音声合成は背景スレッドで行う。
pub struct SynthEngine {
    synth: Mutex<Synthesizer<OpenJtalk>>,
    pub characters: Vec<CharacterInfo>,
}

fn asset_base() -> PathBuf {
    if let Some(path) = std::env::var_os("VOICEVOX_ASSET_DIR") {
        return PathBuf::from(path);
    }
    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."));
    let beside_exe = exe_dir.join("voicevox_core");
    if beside_exe.is_dir() {
        return beside_exe;
    }
    let project = Path::new(env!("CARGO_MANIFEST_DIR")).join("voicevox_core");
    if project.is_dir() { project } else { beside_exe }
}

fn find_dict_dir(base: &Path) -> Option<PathBuf> {
    for candidate in [
        base.join("dict"),
        base.join("dict").join("open_jtalk_dic_utf_8-1.11"),
        base.join("open_jtalk_dic_utf_8-1.11"),
    ] {
        if candidate.join("sys.dic").is_file() {
            return Some(candidate);
        }
    }
    None
}

impl SynthEngine {
    pub fn load() -> anyhow::Result<Self> {
        let base = asset_base();
        let ort_path = base
            .join("onnxruntime")
            .join("lib")
            .join(Onnxruntime::LIB_RECOMMENDED_VERSIONED_FILENAME);
        let ort = Onnxruntime::load_once()
            .filename(ort_path.as_os_str())
            .perform()
            .with_context(|| format!("ONNX Runtime を開けません: {}", ort_path.display()))?;

        let dic_dir = find_dict_dir(&base).ok_or_else(|| {
            anyhow!("OpenJtalk 辞書（sys.dic を含むフォルダ）が見つかりません: {}", base.display())
        })?;
        let dic_str = dic_dir.to_string_lossy();
        let open_jtalk = OpenJtalk::new(&*dic_str)
            .with_context(|| format!("OpenJtalk 辞書を開けません: {}", dic_dir.display()))?;

        let synth = Synthesizer::builder(ort)
            .text_analyzer(open_jtalk)
            .build()
            .context("Synthesizer の構築に失敗")?;

        let models_dir = base.join("models");
        let vvm_files = find_vvm_files(&models_dir)?;
        if vvm_files.is_empty() {
            return Err(anyhow!("ボイスモデル（.vvm）が1つもありません: {}", models_dir.display()));
        }
        for vvm in &vvm_files {
            let model = VoiceModelFile::open(vvm)
                .with_context(|| format!("モデルを開けません: {}", vvm.display()))?;
            if !model.metas().iter().flat_map(|c| &c.styles).any(|s| is_talk_style(s.r#type)) {
                continue;
            }
            synth
                .load_voice_model(&model)
                .perform()
                .with_context(|| format!("モデルを読み込めません: {}", vvm.display()))?;
        }

        let mut characters = Vec::new();
        for character in synth.metas() {
            let styles = character.styles.iter()
                .filter(|style| is_talk_style(style.r#type))
                .map(|style| StyleInfo {
                    name: style.name.clone(),
                    id: style.id.0,
                })
                .collect::<Vec<_>>();
            if !styles.is_empty() {
                characters.push(CharacterInfo {
                    name: character.name.clone(),
                    styles,
                });
            }
        }
        if characters.is_empty() {
            return Err(anyhow!("読み込んだモデルにキャラクターが1つもありません"));
        }
        Ok(Self { synth: Mutex::new(synth), characters })
    }

    pub fn synthesize(&self, text: &str, style_id: u32, speed_scale: f64) -> anyhow::Result<Vec<u8>> {
        let synth = self.synth.lock().map_err(|_| anyhow!("Synthesizer のロックが壊れています"))?;
        let style = StyleId::new(style_id);
        let mut query = synth.create_audio_query(text, style)
            .context("AudioQuery の生成に失敗")?;
        query.speed_scale = PositiveFinite::new(speed_scale as f32)
            .map_err(|_| anyhow!("速度が不正です: {speed_scale}"))?;
        let wav = synth.synthesis(&query, style)
            .perform()
            .context("音声合成に失敗")?;
        Ok(wav)
    }
}

fn is_talk_style(style: StyleType) -> bool {
    matches!(style, StyleType::Talk | StyleType::StreamingTalk)
}

fn find_vvm_files(models_dir: &Path) -> anyhow::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    let nested = models_dir.join("vvms");
    for dir in [nested.as_path(), models_dir] {
        if !dir.is_dir() { continue; }
        for entry in std::fs::read_dir(dir)? {
            let path = entry?.path();
            if path.is_file() && path.extension().is_some_and(|x| x.eq_ignore_ascii_case("vvm")) {
                files.push(path);
            }
        }
        if !files.is_empty() { break; }
    }
    files.sort();
    Ok(files)
}

pub fn save_wav(wav: &[u8]) -> anyhow::Result<PathBuf> {
    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."));
    let out_dir = exe_dir.join("output");
    std::fs::create_dir_all(&out_dir)?;
    save_wav_in(&out_dir, wav)
}

fn save_wav_in(dir: &Path, wav: &[u8]) -> anyhow::Result<PathBuf> {
    use std::io::{ErrorKind, Write};
    for i in 1..=999_999 {
        let path = dir.join(format!("voice_{i:03}.wav"));
        match std::fs::OpenOptions::new().write(true).create_new(true).open(&path) {
            Ok(mut file) => {
                if let Err(err) = file.write_all(wav) {
                    drop(file);
                    let _ = std::fs::remove_file(&path);
                    return Err(err.into());
                }
                return Ok(path);
            }
            Err(err) if err.kind() == ErrorKind::AlreadyExists => continue,
            Err(err) => return Err(err.into()),
        }
    }
    Err(anyhow!("WAV保存用の空きファイル名がありません"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn official_and_legacy_model_paths() -> anyhow::Result<()> {
        let temp = tempfile::tempdir()?;
        let models = temp.path().join("models");
        std::fs::create_dir_all(models.join("vvms"))?;
        std::fs::write(models.join("old.vvm"), [])?;
        assert_eq!(find_vvm_files(&models)?, vec![models.join("old.vvm")]);
        std::fs::write(models.join("vvms/1.vvm"), [])?;
        std::fs::write(models.join("vvms/0.vvm"), [])?;
        std::fs::write(models.join("vvms/readme.txt"), [])?;
        assert_eq!(find_vvm_files(&models)?, vec![models.join("vvms/0.vvm"), models.join("vvms/1.vvm")]);
        Ok(())
    }

    #[test]
    fn wav_save_does_not_overwrite() -> anyhow::Result<()> {
        let temp = tempfile::tempdir()?;
        let first = save_wav_in(temp.path(), b"first")?;
        let second = save_wav_in(temp.path(), b"second")?;
        assert_ne!(first, second);
        assert_eq!(std::fs::read(first)?.as_slice(), b"first");
        assert_eq!(std::fs::read(second)?.as_slice(), b"second");
        Ok(())
    }

    #[test]
    #[ignore = "requires downloaded VOICEVOX runtime, dictionary and VVMs"]
    fn voicevox_smoke() -> anyhow::Result<()> {
        use rodio::Source;
        let engine = SynthEngine::load()?;
        let character = engine.characters.iter().find(|c| c.name == "ずんだもん")
            .context("ずんだもんのモデルがありません")?;
        let style = character.styles.iter().find(|s| s.name == "ノーマル")
            .context("ノーマルスタイルがありません")?;
        let text = "こんにちは、ずんだもんなのだ。";
        let temp = tempfile::tempdir()?;
        let mut lengths = Vec::new();
        for speed in [0.8, 1.2] {
            let wav = engine.synthesize(text, style.id, speed)?;
            assert!(wav.len() > 44 && &wav[..4] == b"RIFF" && &wav[8..12] == b"WAVE");
            let decoder = rodio::Decoder::new(std::io::Cursor::new(wav.clone()))?;
            assert!(decoder.sample_rate() > 0 && decoder.channels() > 0);
            let count = decoder.count();
            assert!(count > 0);
            lengths.push(count);
            let path = save_wav_in(temp.path(), &wav)?;
            assert_eq!(std::fs::read(path)?, wav);
            println!("speed={speed}, samples={count}");
        }
        assert!(lengths[0] > lengths[1], "速度 0.8 の方が長くなるはず");
        let wav = engine.synthesize(text, style.id, 1.0)?;
        let path = save_wav(&wav)?;
        println!("smoke WAV: {}", path.display());
        Ok(())
    }
}
