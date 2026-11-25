//! UIの状態と振る舞い。副作用はEffectとしてシェルに委譲する。
use std::sync::Arc;
use crate::engine::{CharacterInfo, StyleInfo, SynthEngine};

pub enum Effect {
    LoadEngine,
    SynthesizePlay {
        engine: Arc<SynthEngine>,
        text: String,
        style_id: u32,
        speed: f64,
    },
    SaveWav { wav: Arc<Vec<u8>> },
}

pub struct ViewModel {
    engine: Option<Arc<SynthEngine>>,
    pub characters: Vec<CharacterInfo>,
    pub selected_character: String,
    pub selected_style: u32,
    pub text: String,
    pub speed: f64,
    pub busy: bool,
    pub last_wav: Option<Arc<Vec<u8>>>,
    pub status: String,
}

impl ViewModel {
    pub fn new() -> (Self, Effect) {
        (
            Self {
                engine: None,
                characters: Vec::new(),
                selected_character: String::new(),
                selected_style: 0,
                text: String::new(),
                speed: 1.0,
                busy: false,
                last_wav: None,
                status: "エンジンを読み込み中…".to_string(),
            },
            Effect::LoadEngine,
        )
    }

    pub fn ready(&self) -> bool {
        self.engine.is_some()
    }

    pub fn current_styles(&self) -> &[StyleInfo] {
        self.characters.iter()
            .find(|c| c.name == self.selected_character)
            .map(|c| c.styles.as_slice())
            .unwrap_or(&[])
    }

    pub fn selected_style_name(&self) -> &str {
        self.current_styles().iter()
            .find(|s| s.id == self.selected_style)
            .map(|s| s.name.as_str())
            .unwrap_or("（なし）")
    }

    pub fn can_speak(&self) -> bool {
        self.ready() && !self.busy && !self.text.trim().is_empty()
    }

    pub fn can_save(&self) -> bool {
        self.ready() && !self.busy && self.last_wav.is_some()
    }

    pub fn on_engine_ready(&mut self, engine: Arc<SynthEngine>) {
        self.characters = engine.characters.clone();
        let initial = self.characters.iter()
            .find(|c| c.name == "ずんだもん")
            .or_else(|| self.characters.first());
        if let Some(character) = initial {
            self.selected_character = character.name.clone();
            self.selected_style = default_style_id(&character.styles);
        }
        self.engine = Some(engine);
        self.status = "準備完了".to_string();
    }

    pub fn on_engine_failed(&mut self, err: String) {
        self.status = format!("エンジン読み込み失敗: {err}");
    }

    pub fn on_speak_done(&mut self, wav: Vec<u8>) {
        self.busy = false;
        self.last_wav = Some(Arc::new(wav));
        self.status = "再生しました（WAV保存できます）".to_string();
    }

    pub fn on_playback_failed(&mut self, wav: Vec<u8>, err: String) {
        self.busy = false;
        self.last_wav = Some(Arc::new(wav));
        self.status = format!("再生失敗（WAV保存は可能）: {err}");
    }

    pub fn on_speak_failed(&mut self, err: String) {
        self.busy = false;
        self.status = format!("合成失敗: {err}");
    }

    pub fn on_saved(&mut self, path: String) {
        self.busy = false;
        self.status = format!("保存しました: {path}");
    }

    pub fn on_save_failed(&mut self, err: String) {
        self.busy = false;
        self.status = format!("保存失敗: {err}");
    }

    pub fn on_text_changed(&mut self, text: String) {
        self.text = text;
    }

    pub fn on_character_selected(&mut self, name: &str) {
        if let Some(character) = self.characters.iter().find(|c| c.name == name) {
            self.selected_character = character.name.clone();
            self.selected_style = default_style_id(&character.styles);
        }
    }

    pub fn on_style_selected(&mut self, id: u32) {
        if self.current_styles().iter().any(|style| style.id == id) {
            self.selected_style = id;
        }
    }

    pub fn on_speed_changed(&mut self, value: Option<f64>) {
        if let Some(v) = value.filter(|v| v.is_finite()) {
            self.speed = v.clamp(0.5, 2.0);
        }
    }

    pub fn on_speak(&mut self) -> Option<Effect> {
        if self.busy || self.text.trim().is_empty() {
            return None;
        }
        let engine = self.engine.clone()?;
        self.busy = true;
        self.status = "合成中…".to_string();
        Some(Effect::SynthesizePlay {
            engine,
            text: self.text.clone(),
            style_id: self.selected_style,
            speed: self.speed,
        })
    }

    pub fn on_save(&mut self) -> Option<Effect> {
        if self.busy {
            return None;
        }
        match self.last_wav.clone() {
            Some(wav) => {
                self.busy = true;
                self.status = "保存中…".to_string();
                Some(Effect::SaveWav { wav })
            }
            None => {
                self.status = "先に読み上げてください（保存できるWAVがまだありません）".to_string();
                None
            }
        }
    }
}

fn default_style_id(styles: &[StyleInfo]) -> u32 {
    styles.iter()
        .find(|s| s.name == "ノーマル")
        .or_else(|| styles.first())
        .map(|s| s.id)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn background_effect_is_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Effect>();
    }

    #[test]
    fn initial_state_and_speed_limits() {
        let (mut vm, effect) = ViewModel::new();
        assert!(matches!(effect, Effect::LoadEngine));
        assert!(!vm.can_speak());
        assert!(!vm.can_save());
        assert!(vm.on_speak().is_none());
        vm.on_speed_changed(Some(9.0));
        assert_eq!(vm.speed, 2.0);
        vm.on_speed_changed(Some(-1.0));
        assert_eq!(vm.speed, 0.5);
        vm.on_speed_changed(Some(f64::NAN));
        assert_eq!(vm.speed, 0.5);
    }

    #[test]
    fn default_style_and_invalid_selection() {
        let (mut vm, _) = ViewModel::new();
        vm.characters = vec![CharacterInfo {
            name: "ずんだもん".into(),
            styles: vec![StyleInfo { name: "あまあま".into(), id: 1 },
                         StyleInfo { name: "ノーマル".into(), id: 3 }],
        }];
        vm.on_character_selected("ずんだもん");
        assert_eq!(vm.selected_style, 3);
        vm.on_style_selected(999);
        assert_eq!(vm.selected_style, 3);
        vm.on_style_selected(1);
        assert_eq!(vm.selected_style, 1);
    }

    #[test]
    fn playback_error_keeps_wav() {
        let (mut vm, _) = ViewModel::new();
        vm.busy = true;
        vm.on_playback_failed(vec![1, 2, 3], "no device".into());
        assert!(!vm.busy);
        assert_eq!(vm.last_wav.as_deref().unwrap().as_slice(), &[1, 2, 3]);
    }
}
