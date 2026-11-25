use std::io::Cursor;
use anyhow::Context as _;

pub fn play_wav(wav: Vec<u8>) -> anyhow::Result<()> {
    let (_stream, handle) =
        rodio::OutputStream::try_default().context("音声出力デバイスを開けません")?;
    let sink = rodio::Sink::try_new(&handle).context("再生シンクを作れません")?;
    let decoder = rodio::Decoder::new(Cursor::new(wav)).context("WAV をデコードできません")?;
    sink.append(decoder);
    sink.sleep_until_end();
    Ok(())
}
