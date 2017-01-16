# 概要

https://flowtype.org/

JavaScript を静的に型チェックしてくれる

## Type inference
Flow は型注釈なしで、型推論する  
プログラムを実行してるかのように正確に変数の型を追跡する


## Idiomatic JS
Flow は JavaScript プログラマー向けに設計されている  
一般的な JavaScript のイディオムや動的なコードも理解してくれる


## Realtime Feedback
Flow は変更を検知し、プレーンな JavaScript 開発の高速なフィードバックサイクルを提供する


Flow は JavaScript を実行する前に一般的なバグを見つけてくれる
- 暗黙の型変換
- null 参照
- undefined な関数の呼び出し

```javascript
// @flow
function foo(x) {
  return x * 10;
}
foo('Hello, world!');
```

Flow の型注釈の付いた JavaScript コードは簡単に一般的な JavaScript に変換できるので、どこでも動作する

下記のようなコードは Flow ではエラーになるが、実行は可能

```javascript
/* @flow */
var str: number = 'hello world!';
console.log(str);
```

Flow の結果

```bash
src/sample.js:3
  3: var str: number = 'hello world!';
                       ^^^^^^^^^^^^^^ string. This type is incompatible with
  3: var str: number = 'hello world!';
              ^^^^^^ number
```

実行結果

```bash
$ babel-node src/sample.js
hello world!
```

このように Flow は実行を妨げる訳ではない
タイプエラーがある時はプロジェクトを公開しない方がいいが、  
開発時には普通にコードを実行でき、完全なタイプチェックはデバッグやテストの時に行えばよい
Flow はこのように設計されていて、開発中の必要な時にしか干渉しない
