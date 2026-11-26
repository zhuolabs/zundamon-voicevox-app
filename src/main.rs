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
    CharacterSelected(Option<String>),
    StyleSelected(Option<u32>),
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
            Message::CharacterSelected(Some(name)) => self.vm.on_character_selected(&name),
            Message::StyleSelected(Some(id)) => self.vm.on_style_selected(id),
            Message::CharacterSelected(None) | Message::StyleSelected(None) => {},
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
        context.window_visuals(WindowVisuals::new().client_size(480.0, 420.0));
        let vm = &self.vm;
        let interactive = vm.ready() && !vm.busy;
        let character_names: Vec<String> = vm.characters.iter()
            .map(|character| character.name.clone()).collect();
        let character_index = character_names.iter()
            .position(|name| name == &vm.selected_character);
        let style_names: Vec<String> = vm.current_styles().iter()
            .map(|style| style.name.clone()).collect();
        let style_ids: Vec<u32> = vm.current_styles().iter()
            .map(|style| style.id).collect();
        let style_index = style_ids.iter().position(|id| *id == vm.selected_style);
        // The callbacks map the index in the rendered list to its stable domain value.
        // Ignore the temporary None notification when a ComboBox list is replaced.
        let character_values = character_names.clone();
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
                            .font_size(14.0)
                            .height(94.0)
                            .is_enabled(interactive)
                            .on_text_changed(context.callback(|value: String| {
                                Message::TextChanged(value)
                            })),
                        TextBlock::new().text("キャラクター"),
                        ComboBox::new()
                            .items_source(character_names)
                            .selected_index(character_index)
                            .placeholder_text("キャラクターを選択")
                            .horizontal_alignment(HorizontalAlignment::Stretch)
                            .is_enabled(interactive)
                            .on_selection_changed(context.callback(move |index: Option<usize>| {
                                Message::CharacterSelected(index.and_then(|i| character_values.get(i).cloned()))
                            })),
                        TextBlock::new().text("スタイル"),
                        ComboBox::new()
                            .items_source(style_names)
                            .selected_index(style_index)
                            .placeholder_text("スタイルを選択")
                            .horizontal_alignment(HorizontalAlignment::Stretch)
                            .is_enabled(interactive)
                            .on_selection_changed(context.callback(move |index: Option<usize>| {
                                Message::StyleSelected(index.and_then(|i| style_ids.get(i).copied()))
                            })),
                        TextBlock::new().text("速度"),
                        NumberBox::new()
                            .minimum(0.5)
                            .maximum(2.0)
                            .value(Some(vm.speed))
                            .is_enabled(interactive)
                            .on_value_changed(context.callback(Message::SpeedChanged)),
                        StackPanel::new()
                            .orientation(Orientation::Horizontal)
                            .spacing(8.0)
                            .children((
                                Button::new()
                                    .is_enabled(vm.can_speak())
                                    .on_click(context.message(Message::Speak))
                                    .content("読み上げ"),
                                Button::new()
                                    .is_enabled(vm.can_save())
                                    .on_click(context.message(Message::Save))
                                    .content("WAV保存"),
                            )),
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
