#![cfg_attr(not(test), windows_subsystem = "windows")]

mod engine;
mod playback;
mod viewmodel;

use std::sync::Arc;
use windows_reactor::*;
use engine::SynthEngine;
use viewmodel::{Effect, ViewModel};

#[derive(Clone)]
enum Message {
    EngineReady(Arc<SynthEngine>),
    EngineFailed(String),
    TextChanged(String),
    CharacterSelected(String),
    StyleSelected(u32),
    SpeedChanged(Option<f64>),
    Speak,
    SpeakDone(Vec<u8>),
    SpeakFailed(String),
    PlaybackFailed(Vec<u8>, String),
    Save,
    Saved(String),
    SaveFailed(String),
}

struct VoiceApp {
    vm: ViewModel,
}

/// ViewModel の Effect を背景スレッドで実行し、結果を Message で戻す。
fn run_effect(context: &ComponentContext<VoiceApp>, effect: Effect) {
    match effect {
        Effect::LoadEngine => {
            _ = context.spawn_background(|_| match SynthEngine::load() {
                Ok(engine) => Message::EngineReady(Arc::new(engine)),
                Err(err) => Message::EngineFailed(format!("{err:#}")),
            });
        }
        Effect::SynthesizePlay { engine, text, style_id, speed } => {
            _ = context.spawn_background(move |_| {
                match engine.synthesize(&text, style_id, speed) {
                    Ok(wav) => {
                        if let Err(err) = playback::play_wav(wav.clone()) {
                            return Message::PlaybackFailed(wav, format!("{err:#}"));
                        }
                        Message::SpeakDone(wav)
                    }
                    Err(err) => Message::SpeakFailed(format!("{err:#}")),
                }
            });
        }
        Effect::SaveWav { wav } => {
            _ = context.spawn_background(move |_| match engine::save_wav(&wav) {
                Ok(path) => Message::Saved(path.display().to_string()),
                Err(err) => Message::SaveFailed(format!("{err:#}")),
            });
        }
    }
}

impl Component for VoiceApp {
    type Input = ();
    type Message = Message;

    fn create(_input: &(), context: &ComponentContext<Self>) -> Self {
        let (vm, effect) = ViewModel::new();
        run_effect(context, effect);
        Self { vm }
    }

    fn update(&mut self, message: Message, context: &ComponentContext<Self>) {
        match message {
            Message::EngineReady(engine) => self.vm.on_engine_ready(engine),
            Message::EngineFailed(err) => self.vm.on_engine_failed(err),
            Message::TextChanged(text) => self.vm.on_text_changed(text),
            Message::CharacterSelected(name) => self.vm.on_character_selected(&name),
            Message::StyleSelected(id) => self.vm.on_style_selected(id),
            Message::SpeedChanged(value) => self.vm.on_speed_changed(value),
            Message::Speak => {
                if let Some(effect) = self.vm.on_speak() {
                    run_effect(context, effect);
                }
            }
            Message::SpeakDone(wav) => self.vm.on_speak_done(wav),
            Message::SpeakFailed(err) => self.vm.on_speak_failed(err),
            Message::PlaybackFailed(wav, err) => self.vm.on_playback_failed(wav, err),
            Message::Save => {
                if let Some(effect) = self.vm.on_save() {
                    run_effect(context, effect);
                }
            }
            Message::Saved(path) => self.vm.on_saved(path),
            Message::SaveFailed(err) => self.vm.on_save_failed(err),
        }
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("VOICEVOX 音声合成");
        let vm = &self.vm;
        let interactive = vm.ready() && !vm.busy;
        let character_buttons: Vec<_> = vm.characters.iter().map(|character| {
            let name = character.name.clone();
            KeyedView::new(
                format!("character-{name}"),
                Button::new()
                    .is_enabled(interactive && vm.selected_character != name)
                    .on_click(context.message(Message::CharacterSelected(name.clone())))
                    .content(name),
            )
        }).collect();
        let style_buttons: Vec<_> = vm.current_styles().iter().map(|style| {
            let id = style.id;
            KeyedView::new(
                format!("style-{id}"),
                Button::new()
                    .is_enabled(interactive && vm.selected_style != id)
                    .on_click(context.message(Message::StyleSelected(id)))
                    .content(style.name.clone()),
            )
        }).collect();
        let content = Border::new()
            .padding(Thickness::uniform(16.0))
            .content(
                StackPanel::new()
                    .spacing(8.0)
                    .children((
                        TextBlock::new().text("VOICEVOX 音声合成").font_size(20.0),
                        TextBlock::new().text(vm.status.clone()).text_wrapping(TextWrapping::Wrap),
                        TextBlock::new().text("読ませたい文字"),
                        TextBox::new()
                            .text(vm.text.clone())
                            .placeholder_text("ここに入力した文字を読み上げます")
                            .accepts_return(true)
                            .text_wrapping(TextWrapping::Wrap)
                            .height(120.0)
                            .is_enabled(interactive)
                            .on_text_changed(context.callback(|value: String| {
                                Message::TextChanged(value)
                            })),
                        TextBlock::new()
                            .text(format!("キャラクター: {}", vm.selected_character)),
                        ScrollViewer::new()
                            .height(160.0)
                            .vertical_scroll_bar_visibility(ScrollBarVisibility::Auto)
                            .content(StackPanel::new().spacing(4.0).keyed_children(character_buttons)),
                        TextBlock::new()
                            .text(format!("スタイル: {}", vm.selected_style_name())),
                        ScrollViewer::new()
                            .height(120.0)
                            .vertical_scroll_bar_visibility(ScrollBarVisibility::Auto)
                            .content(StackPanel::new().spacing(4.0).keyed_children(style_buttons)),
                        TextBlock::new().text("速度"),
                        NumberBox::new()
                            .minimum(0.5)
                            .maximum(2.0)
                            .value(Some(vm.speed))
                            .is_enabled(interactive)
                            .on_value_changed(context.callback(Message::SpeedChanged)),
                        Button::new()
                            .is_enabled(vm.can_speak())
                            .on_click(context.message(Message::Speak))
                            .content("読み上げ"),
                        Button::new()
                            .is_enabled(vm.can_save())
                            .on_click(context.message(Message::Save))
                            .content("WAV保存"),
                    )),
            );
        ScrollViewer::new()
            .vertical_scroll_bar_visibility(ScrollBarVisibility::Auto)
            .content(content)
            .into()
    }
}

fn main() {
    windows_reactor::App::run_component::<VoiceApp>(()).unwrap();
}
