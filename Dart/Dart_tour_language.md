# A Tour of the Dart Language

このページでは、すでに別の言語でプログラミングする方法を前提として、Dartの各主要な機能、変数、演算子、クラス、ライブラリなどの使用方法を説明する  

言語機能の詳細については、[Dart言語仕様](http://www.ecma-international.org/publications/files/ECMA-ST/ECMA-408.pdf)を参照

## A basic Dart program

以下のコードは、Dartの最も基本的な機能の多くを使用している

```dart

// Define a function.
printNumber(num aNumber) {
  print('The number is $aNumber.'); // Print to console.
}

// This is where the app starts executing.
main() {
  var number = 42; // Declare and initialize a variable.
  printNumber(number); // Call a function.
}
```

このプログラムが使用しているものは全てのDartアプリケーションに適用できる

### // This is a comment.

`//`を使うと、それ以降がコメントになる。あるいは、`/* ... */`。詳細は[ここ](https://www.dartlang.org/guides/language/language-tour#comments)を参照

### num

型。他のビルトインの型として、`String`、`int`、`bool`などがある

### 42

数値リテラル。数値リテラルはコンパイル時定数の一種

### print()

出力を表示するのに便利な方法

### '...' or "..."

文字列リテラル

### $変数名 or ${変数名}

文字列インターポレート：変数や文字列リテラルの内部に相当する式の文字列を含む。詳細は[ここ](https://www.dartlang.org/guides/language/language-tour#strings)

### main()

アプリケーションの実行に必須な特別なトップレベル関数。詳細は[ここ](https://www.dartlang.org/guides/language/language-tour#the-main-function)

### var

型指定なしの変数宣言の方法

## Important concepts

Dartを学ぶ時に念頭に置いておく以下の事実と概念

- 変数は全てオブジェクトであり、全てのオブジェクトはクラスのインスタンス。数値でさえ、関数でさえ、`null`でさえ。全てのオブジェクトは`Object`クラスを継承している

- 静的に型を指定すると意図が明確になり、ツールによる静的チェックが可能になる。静的に型を指定することは、1.xではオプションだが、完全に型を指定する方向に移行している

- `strong`モードでは、静的チェックとランタイムチェックにより、コードがタイプセーフであることが保証され、実行時ではなく開発時のバグを検出できる。strongモードは1.xではオプションだが、2.0ではオプションではない

- Dartは実行する前に全てのコードを解析する。Dartはエラーをキャッチするヒントを提供したり、コードをより速く実行する手助けをする

- Dartはトップレベル関数(main関数)やクラスやオブジェクトに関連する関数(静的メソッド、インスタンスメソッド)をサポートしている

- 同様に、Dartはトップレベルの変数だけでなく、クラスやオブジェクト(静的変数、インスタンス変数)に関連付けられた変数もサポートしている。インスタンス変数は、フィールドまたはプロパティと呼ばれることもある

- Javaと異なり、`public`、`protected`、`private`というキーワードを持たない。変数名がアンダースコア(_)で始まる場合、そのライブラリのプライベートになる。詳細は[ここ](https://www.dartlang.org/guides/language/language-tour#libraries-and-visibility)

- 変数名は文字または_で始まり、その後にそれらの文字と数字の組み合わせが続く

- 式か文かが重要で、それらを明確にする必要がある

- Dartツールは2つの問題、「警告」と「エラー」をレポートする。警告はプログラムの実行を妨げるものではないが、コード的に正しくないことを示す。エラーはコンパイル時、実行時のいずれかで起こる。コンパイル時のエラーはコードの実行を妨げる。実行時のエラーはコードの実行中に例外が発生する

- 1.xには2つのランタイムモードがあり、「production」と「checked」である。我々は、開発時・デバッグ時には「checked」モード、デプロイ時には「production」モードを推奨する。productionモードはデフォルトの実行時モードであり、速度に最適化する。productionモードでは、`assert`文と静的な型を無視する。checkedモードディベロッパーに優しいモードで、実行時に発生したエラーのタイプをわかりやすくする。例えば、`num`型で宣言した変数に数値じゃないものを割り当てると例外を発生させる