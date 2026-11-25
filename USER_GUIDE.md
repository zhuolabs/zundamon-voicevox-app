# ずんだもん音声合成アプリ — Windows x64 使用説明書

## 1. ダウンロード

GitHub [Actions](https://github.com/zhuolabs/zundamon-voicevox-app/actions) の成功した Windows build and verification の実行結果を開き、Artifacts にある zundamon-voicevox-app-windows-x64 をダウンロードして ZIP をすべて展開します。GitHub へのログインが必要な場合があります。モデルや辞書は同梱しません。EXE 単体ではなく、同梱されたランタイムファイルも同じ場所に残してください。

Windows 11 x64 を推奨します。Rust や Visual Studio はアプリ利用時には不要です。MSVC ランタイム不足の場合は Microsoft 公式の Visual C++ 再頒布可能パッケージ x64 を導入してください: https://learn.microsoft.com/cpp/windows/latest-supported-vc-redist

## 2. モデルなどの取得（初回のみ）

展開したフォルダで PowerShell を開き、次を実行します。

~~~powershell
powershell -ExecutionPolicy Bypass -File .\setup-assets.ps1
~~~

VOICEVOX Core 0.17.1 の公式ダウンローダーから音声モデル、OpenJTalk 辞書、ONNX Runtime を取得します。インターネット接続と十分な空き容量が必要です。入手元: https://github.com/VOICEVOX/voicevox_core/releases/tag/0.17.1

ダウンローダーに表示される音声モデルと ONNX Runtime の利用規約を読み、同意できる場合だけ y を入力してください。

アセットは EXE の隣の voicevox_core フォルダに保存されます。破損等で再取得する場合は同じコマンドに -Force を付けます。

## 3. 起動と使い方

zundamon-voicevox-app.exe を起動し、モデル読み込みが完了するまで待ちます。キャラクターとして「ずんだもん」、スタイルとして「ノーマル」を選び、文章を入力し「読み上げ」を押します。速度を変更できます。読み上げ後に「WAV保存」を押すと、EXE の隣の output/voice_NNN.wav に保存されます。既存の WAV は上書きしません。

## 4. トラブルシュート

- モデルが見つからない: voicevox_core が EXE の隣にあることを確認し、セットアップを再実行してください。
- DLL が見つからない: 配布 ZIP を丸ごと展開したこと、x64 Windows を使用していることを確認してください。
- 音が出ない: Windows の出力デバイスと音量を確認してください。WAV 保存も試してください。
- 不具合報告: Actions の実行 URL、Windows バージョン、表示されたエラー、操作手順を添えてください。

## 5. 利用条件・検証範囲

音声の利用には各キャラクター・モデルの規約が適用されます。必要なクレジット（例: VOICEVOX:ずんだもん）を表示してください。VOICEVOX の公式サイト: https://voicevox.hiroshiba.jp/

CI はコンパイル、単体テスト、release EXE の生成を検証します。実モデルによる WAV 合成・デコード・保存は、利用規約への同意が明示された場合のみ実施します。GUI 操作および実際のスピーカー出力は FIRST_RUN_TESTS.md に沿って実機で確認してください。CI が成功するまでは動作確認済みの配布物ではありません。
