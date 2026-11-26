# ずんだもん音声合成アプリ — Windows x64 使用説明書

## 1. ダウンロード

GitHub [Actions](https://github.com/zhuolabs/zundamon-voicevox-app/actions) の成功した Windows build and verification の実行結果を開き、Artifacts にある zundamon-voicevox-app-windows-x64 をダウンロードして ZIP をすべて展開します。GitHub へのログインが必要な場合があります。音声モデル・辞書・VOICEVOX ONNX Runtime は同梱しません。公式ダウンローダー download.exe は同梱します。EXE 単体ではなく、同梱されたランタイムファイルも同じ場所に残してください。

Windows 11 x64 を推奨します。Rust や Visual Studio はアプリ利用時には不要です。MSVC ランタイム不足の場合は Microsoft 公式の Visual C++ 再頒布可能パッケージ x64 を導入してください: https://learn.microsoft.com/cpp/windows/latest-supported-vc-redist

## 2. モデルなどの取得（初回のみ）

展開したフォルダで **setup-assets.cmd をダブルクリック**してください。PowerShell から実行する場合は以下でも可能です。

~~~powershell
powershell -ExecutionPolicy Bypass -File .\setup-assets.ps1
~~~

同梱の VOICEVOX Core 0.17.1 公式ダウンローダー download.exe を実行し、読み上げ用音声モデル、OpenJTalk 辞書、ONNX Runtime を取得します。インターネット接続と十分な空き容量が必要です。入手元: https://github.com/VOICEVOX/voicevox_core/releases/tag/0.17.1

ダウンローダーに表示される音声モデルと ONNX Runtime の利用規約を読み、同意できる場合だけ y を入力してください。

アセットは EXE の隣の voicevox_core フォルダに保存されます。破損等で再取得する場合はPowerShell のコマンドに -Force を付けます。直接ダウンローダーを実行する場合は、展開先で ` .\download.exe -o voicevox_core --exclude c-api --devices cpu --models-pattern '[0-9]*.vvm' ` を実行してください。

## 3. 起動と使い方

zundamon-voicevox-app.exe を起動し、モデル読み込みが完了するまで待ちます。ドロップダウンでキャラクター「ずんだもん」、スタイル「ノーマル」を選び、文章を入力し「読み上げ」を押します。速度を変更できます。テキスト欄は日本語約30文字を1行の目安とし、複数行入力にも対応します（文字数制限なし）。読み上げ後に「WAV保存」を押すと、EXE の隣の output/voice_NNN.wav に保存されます。既存の WAV は上書きしません。

## 4. トラブルシュート

- モデルが見つからない: voicevox_core が EXE の隣にあることを確認し、セットアップを再実行してください。
- DLL が見つからない: 配布 ZIP を丸ごと展開したこと、x64 Windows を使用していることを確認してください。
- 音が出ない: Windows の出力デバイスと音量を確認してください。WAV 保存も試してください。
- 不具合報告: Actions の実行 URL、Windows バージョン、表示されたエラー、操作手順を添えてください。

## 5. 利用条件・検証範囲

同梱の公式ダウンローダーの上流ライセンスは VOICEVOX-DOWNLOADER-LICENSE.txt を参照してください。

音声の利用には各キャラクター・モデルの規約が適用されます。必要なクレジット（例: VOICEVOX:ずんだもん）を表示してください。VOICEVOX の公式サイト: https://voicevox.hiroshiba.jp/

CI はコンパイル、単体テスト、release EXE の生成を検証します。CI ではモデルを取得せず、実音声合成テストも実施しません。音声合成・GUI 操作・実際のスピーカー出力は FIRST_RUN_TESTS.md に沿って実機で確認してください。CI が成功するまでは動作確認済みの配布物ではありません。
