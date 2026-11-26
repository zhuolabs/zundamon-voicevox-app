# VOICEVOX 音声合成アプリ（Rust × Windows Reactor）

Windows Reactor 0.100.0 (WinUI 3)、VOICEVOX Core 0.17.1、rodio 0.20 を使用。元の ViewModel / Effect 構成を維持しています。テキスト入力、キャラクター・読み上げスタイル選択、速度変更、音声再生、WAV 保存に対応する設計です。

従来の self-contained 版は CI #8 でデバッグ・リリースビルド、単体テスト6件、配布アーティファクト生成に成功しました。GUI の実機動作は未確認です。CI の結果は [Actions](https://github.com/zhuolabs/zundamon-voicevox-app/actions) から確認してください。

## アプリが起動しない場合

本アプリは Framework-dependent 方式です。Windows App Runtime の DLL・言語リソースは同梱せず、PC にインストール済みのランタイムを利用します。既に対応ランタイムがある場合は追加操作は不要です。

起動できない場合は、Microsoft 公式の [Windows App SDK ダウンロード](https://learn.microsoft.com/ja-jp/windows/apps/windows-app-sdk/downloads) を開き、Windows App SDK 2.0 系（2.x、Stable）の Runtime downloads から x64 Installer（WindowsAppRuntimeInstall.exe）を取得して実行してください。その後アプリを再起動します。Windows App Runtime は .NET Desktop Runtime とは別のものです。

MSVC ランタイム不足が報告された場合は [Visual C++ 再頒布可能パッケージ x64](https://learn.microsoft.com/ja-jp/cpp/windows/latest-supported-vc-redist) もインストールしてください。ランタイムの自動インストール処理はありません。

## Windows x64 での検証

前提: Windows 11 推奨、Rust stable (x86_64-pc-windows-msvc)、Visual Studio Build Tools の C++ デスクトップ開発（MSVC、Windows SDK）、CMake、Git、インターネット接続。

PowerShell でリポジトリ直下に移動し、以下を実行します。

~~~powershell
powershell -ExecutionPolicy Bypass -File .\verify.ps1 -Launch
~~~

ビルドと単体テストを実行します。アセットは自動取得しません。GUI を起動する前に `setup-assets.cmd` を実行し、利用規約に同意して必要なアセットを取得してください。`-Launch` 指定時は最後に GUI を起動します。

GitHub Actions は push / pull request / 手動実行で Windows x64 のビルド、単体テスト、配布アーティファクト生成を実施します。Windows App Runtime は同梱しません。公式ダウンローダー EXE を同梱しますが、CI では実行せず、音声モデル・辞書・VOICEVOX ONNX Runtime は一切取得・再配布しません。実音声合成は実機で確認します。

run の Artifacts にある zundamon-voicevox-app-windows-x64 が配布物です。初回 CI で生成した Cargo.lock は cargo-lock アーティファクトから取得して固定できます。

## アセット配置

VOICEVOX_ASSET_DIR 環境変数、exe の隣、開発時のプロジェクト直下の順で voicevox_core を検索します。公式ダウンローダーによる配置:

~~~
voicevox_core/
  onnxruntime/lib/<推奨DLL名>
  dict/open_jtalk_dic_utf_8-1.11/sys.dic
  models/vvms/*.vvm
~~~

旧配置 models/*.vvm も対応。読み上げ対応キャラクターから選択可能。歌唱専用スタイルは除外します。

## ファイル

- src/main.rs: Reactor の描画、Effect 実行
- src/viewmodel.rs: 状態と振る舞い、単体テスト
- src/engine.rs: モデル読み込み、音声合成、WAV 保存とテスト
- src/playback.rs: rodio による再生
- setup-assets.cmd / setup-assets.ps1: 利用者による対話式アセット取得
- verify.ps1: ビルドと検証
- package.ps1: 配布用ファイル生成
- [USER_GUIDE.md](USER_GUIDE.md): 使用説明書
- [FIRST_RUN_TESTS.md](FIRST_RUN_TESTS.md): 実機確認手順

## ライセンス・クレジット

同梱する公式ダウンローダーは [VOICEVOX Core 0.17.1](https://github.com/VOICEVOX/voicevox_core/releases/tag/0.17.1) の配布バイナリです。上流の MIT ライセンスを VOICEVOX-DOWNLOADER-LICENSE.txt に同梱しています。

生成音声を使用する際は各キャラクターの規約に従い、必要なクレジット（例: VOICEVOX:ずんだもん）を表示してください。各依存ライブラリ、モデルのライセンスも確認してください。
