# Jest

https://facebook.github.io/jest/

Delightful JavaScript Testing

## Easy Setup

JavaScriptテストソリューションを完全かつ簡単にセットアップできる  
どのようなReactプロジェクトでもすぐに使用できる

## Instant Feedback

高速でインタラクティブな監視モードでは変更されたファイルに関連するテストファイルのみが実行され  
すばやく結果が返されるよう最適化されている

## Snapshot Testing

Reactツリーまたはシリアライズ可能な値のスナップショットをキャプチャしてテストを簡素化し  
時間の経過とともに状態がどのように変化するかを分析する

## Zero configuration testing platform

JestはReactアプリケーションを含む全てのJavaScriptのコードをテストするためにFacebookで使用されている  
Jestの思想の1つは、`zero-configuration`を提供すること  
エンジニアにすぐに使用できるツールが提供されると、より多くのテストが作成され、結果としてより安定した健康的なコードベースが作成されることがわかった

## Fast and sandboxed

Jestはパフォーマンスを最大化するためにワーカー間のテスト実行を並列化する  
コンソールメッセージはバッファリングされ、テスト結果とともに出力される  
テストファイルはサンドボックス化され、テストごとにグローバルの状態が自動的にリセットされるので、テストが競合することはない

## Built-in code coverage reports

`--coverage`を使うことで簡単にコードカバレッジレポートを作成できる  
追加のセットアップやライブラリは必要ない  
Jestは、テストされていないファイルを含め、プロジェクト全体からコードカバレッジ情報を収集することができる

## Zero configuration

Jestは`create-react-app`または`react-native-init`を使用してプロジェクトを作成した時に設定している  
テストを`__tests__`フォルダに置くか、拡張子に`.spec.js`または`.test.js`を付けるだけ  
Jestはそれを見つけて実行してくれる
