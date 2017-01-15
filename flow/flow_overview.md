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

```js
// @flow
function foo(x) {
  return x * 10;
}
foo('Hello, world!');
```

Flow の型注釈の付いた JavaScript コードは簡単に一般的な JavaScript に変換できるので、どこでも動作する
