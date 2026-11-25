# VOICEVOX 音声合成アプリ（Rust × Windows Reactor）

Windows Reactor 0.100.0 (WinUI 3)、VOICEVOX Core 0.17.1、rodio 0.20 を使用。元の ViewModel / Effect 構成を維持しています。テキスト入力、キャラクター・読み上げスタイル選択、速度変更、音声再生、WAV 保存に対応する設計です。

Windows のデバッグビルドおよび単体テスト6件は CI #4 で成功しました。GUI の実機動作は未確認です。CI の結果は [Actions](https://github.com/zhuolabs/zundamon-voicevox-app/actions) から確認してください。

## Windows x64 での検証

前提: Windows 11 推奨、Rust stable (x86_64-pc-windows-msvc)、Visual Studio Build Tools の C++ デスクトップ開発（MSVC、Windows SDK）、CMake、Git、インターネット接続。

PowerShell でリポジトリ直下に移動し、以下を実行します。

~~~powershell
powershell -ExecutionPolicy Bypass -File .\verify.ps1 -Launch
~~~

ビルド、単体テスト、公式アセットの取得、実音声合成・WAV デコードと保存のテストを順に実行します。-Launch 指定時は最後に GUI を起動します。ビルドと単体テストのみなら -SkipAssets を指定してください。

GitHub Actions は push / pull request / 手動実行で Windows x64 ビルド、単体テスト、配布アーティファクト生成を実施します。実モデルの音声合成テストは配布アーティファクト生成後に、利用規約への同意を明示した場合のみ実施します。

VOICEVOX 音声モデルと ONNX Runtime の利用規約を確認して同意した場合は、リポジトリの Settings → Secrets and variables → Actions → Variables で `VOICEVOX_TERMS_ACCEPTED` を `true` にするか、Actions の手動実行で `accept_voicevox_terms` を有効にしてください。規約: https://github.com/VOICEVOX/voicevox_vvm/releases/tag/0.17.0 および https://github.com/VOICEVOX/onnxruntime/releases 。CI は同意入力を標準入力へ渡します。

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
- setup-assets.ps1: アセット取得
- verify.ps1: ビルドと検証
- package.ps1: 配布用ファイル生成
- [USER_GUIDE.md](USER_GUIDE.md): 使用説明書
- [FIRST_RUN_TESTS.md](FIRST_RUN_TESTS.md): 実機確認手順

## ライセンス・クレジット

生成音声を使用する際は各キャラクターの規約に従い、必要なクレジット（例: VOICEVOX:ずんだもん）を表示してください。各依存ライブラリ、モデルのライセンスも確認してください。
