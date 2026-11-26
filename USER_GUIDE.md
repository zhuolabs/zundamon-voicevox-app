# ずんだもん音声合成アプリ — Windows x64 使用説明書

## 1. ダウンロード

GitHub [Actions](https://github.com/zhuolabs/zundamon-voicevox-app/actions) の成功した Windows build and verification の実行結果を開き、Artifacts にある zundamon-voicevox-app-windows-x64 をダウンロードして ZIP をすべて展開します。GitHub へのログインが必要な場合があります。音声モデル・辞書・VOICEVOX ONNX Runtime は同梱しません。公式ダウンローダー download.exe は同梱します。Framework-dependent 方式のため Windows App Runtime は同梱しません。

Windows 11 x64 を推奨します。Rust や Visual Studio はアプリ利用時には不要です。起動できない場合は [README.md の「アプリが起動しない場合」](README.md) を参照してください。

## 2. モデルなどの取得（初回のみ）

[README.md の「初回セットアップ（DirectML版）」](README.md) を参照し、公式ダウンローダーで音声モデル・Open JTalk 辞書・ONNX Runtime・DirectML を取得してください。利用規約への同意は利用者が行います。DirectX 12 対応 GPU が必要です。モデル取得スクリプトは同梱しません。

## 3. 起動と使い方

zundamon-voicevox-app.exe を起動し、モデル読み込みが完了するまで待ちます。ドロップダウンでキャラクター「ずんだもん」、スタイル「ノーマル」を選び、文章を入力し「読み上げ」を押します。速度を変更できます。テキスト欄は日本語約30文字を1行の目安とし、複数行入力にも対応します（文字数制限なし）。読み上げ後に「WAV保存」を押すと、EXE の隣の output/voice_NNN.wav に保存されます。既存の WAV は上書きしません。

## 4. トラブルシュート

- モデルが見つからない: voicevox_core が EXE の隣にあることを確認し、README の初回セットアップ手順を再確認してください。
- 起動できない、DLL が見つからない: x64 Windows を使用していることを確認し、README.md に記載の Microsoft 公式ランタイム入手方法を参照してください。
- GPU 初期化失敗: DirectX 12 対応 GPU、最新のグラフィックスドライバー、DirectML.dll の配置を確認してください。
- 音が出ない: Windows の出力デバイスと音量を確認してください。WAV 保存も試してください。
- 不具合報告: Actions の実行 URL、Windows バージョン、表示されたエラー、操作手順を添えてください。

## 5. 利用条件・検証範囲

同梱の公式ダウンローダーの上流ライセンスは VOICEVOX-DOWNLOADER-LICENSE.txt を参照してください。

音声の利用には各キャラクター・モデルの規約が適用されます。必要なクレジット（例: VOICEVOX:ずんだもん）を表示してください。VOICEVOX の公式サイト: https://voicevox.hiroshiba.jp/

CI はコンパイル、単体テスト、release EXE の生成を検証します。CI ではモデルを取得せず、実音声合成テストも実施しません。音声合成・GUI 操作・実際のスピーカー出力は FIRST_RUN_TESTS.md に沿って実機で確認してください。CI が成功するまでは動作確認済みの配布物ではありません。
