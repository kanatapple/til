# Getting Started

## インストール

### Windows

https://win.rustup.rs/

rustup-init.exe を叩くとコマンドプロンプトが出てくる  
デフォルトインストールするかカスタマイズするか聞かれるので選択する  
%USERPROFILE%\.cargo\bin にバイナリがインストールされるのでパス通ってなければ通す  
rustup にバグがあるのかコンソールを再起動するかログアウトしないとパスが反映されないらしい(2017/01/17現在)

``` bash
$ rustc --version
rustc 1.14.0 (e8a012324 2016-12-16)
```

Windows 環境では C++ build tools for Visual Studio 2013+ も追加で必要

### MacOS or Linux

```bash
$ curl https://sh.rustup.rs -sSf | sh
```

## アンインストール

```bash
$ rustup self uninstall
```

## Hello, world!

新しい言語を学ぶ時に "Hello, world!" とスクリーンに表示する小さなプログラムを書くことが伝統  
単純なプログラムで始めることはコンパイラがインストールされ、それが正しく動くか把握するのによい  
またスクリーンに表示することはよくやることなので、練習しておくといい

### プロジェクト作成

Rust はコードがどこにあるかは気にしない

```bash
$ mkdir hello_world
$ cd hello_world
```

### Rust プログラムの記述と実行

Rust ファイルは `.rs` 拡張子で終わる  
もしファイル名に1単語以上の名前を付ける場合は、アンダースコアで区切る(my_program.rsのように)

下記のコードを記述

```rust
fn main() {
    println!("Hello, world!");
}
```

下記を実行

```bash
$ rustc main.rs
$ ./main
Hello, world!
```

使ってる OS に限らず、ターミナルに Hello, world! と表示されるはず

## Rust プログラムの解剖

"Hello, world!" プログラムで何が起きていたのか見ていく  
ここにパズルの1ピースがある

```rust
fn main() {
}
```

これらの行は Rust における関数を定義している  
main 関数は特別。全ての Rust プログラムの開始地点  
最初の行は、「引数も戻り値も持たない main という関数を定義している」と言っている  
引数がある場合は ( と ) の中に入り、戻り値がない場合は戻り値の型を省略できる

関数の本体は { } で囲まれる  
() の後ろに1つのスペースを置き、同じ行に { を書くことがよいコーディングスタイルとされている

```rust
    println!("Hello, world");
```

この行には重要なことがいくつかある  
1つ目はインデントがタブではなくスペース4つ

2つ目は println!() の部分  
これは Rust のメタプログラミング機構、マクロの呼び出し  
もし関数を呼び出しているなら、println() になる(!がない)  
「!」を見たらマクロを呼び出していると意味している

3つ目は "Hello, world!" の文字列
文字列はシステムプログラミング言語では複雑なトピックで、静的に割り当てたれた文字列

4つ目は行末にあるセミコロン  
Rust は式指向言語で、ほとんどのものは文ではなく式になる  
セミコロンは式が終わり、次の式が始まることを表している

## コンパイルと実行は別の手順

Rust プログラムを実行する前にコンパイルが必要
Rust コンパイラは下記のように入力することで使える

```bash
$ rustc main.rs
```

コンパイルが成功すると実行可能なバイナリファイルが出力される

Rust は、プログラムをコンパイルして、それを別の誰かに渡したら、Rust がなくても動く、事前コンパイル言語(ahead-of-time compiled language)

## Hello, Cargo!

Cargo は Rust のビルドシステムであり、パッケージマネージャでもある

Cargo は3つのことを管理する
- コードのビルド
- 依存しているライブラリのダウンロード
- ライブラリのビルド

Rust のプロジェクトではほとんど Cargo を使う

## Cargo へ変換

### ソースディレクトリを作成

Cargo はソースファイルが src ディレクトリあるものとして動く  
src ディレクトリにコードを移す

### 設定ファイルを作成

プロジェクト直下に Cargo.toml というファイルを作る  
Cargo.toml の C が大文字になってることが重要。こうしないと Cargo が設定ファイルと認識しない  
(って書いてあるけど、小文字でも動いた)
このファイルは TOML フォーマットで記述される  
INI ファイルに似ているがいろいろな機能が追加されている

Cargo.toml に以下の情報を記述

```toml
[package]
name = "hello_world"
version = "0.0.1"
authors = ["名前 <メールアドレス>"]
```

最初の行の `[package]` はパッケージの設定であることを示す  

Cargo がコンパイルする時に必要な情報
- プログラムの名前
- バージョン
- 著者

## Cargo プロジェクトのビルドと実行

ビルドと実行を別々に

```bash
$ cargo build
   Compiling hello_world v0.0.1 (file:///D:/git-repos/til/Rust)
    Finished debug [unoptimized + debuginfo] target(s) in 0.39 secs
    
$ target/debug/hello_world.exe
Hello, workd!
```

ビルドと実行を一発で

```bash
$ cargo run
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target\debug\hello_world.exe`
Hello, world!
```

ソースコードが変更されてない場合はコンパイルが走らない
Cargo は最後にビルドしてから変更されたファイルがある時だけ再度ビルドを行う

## リリースビルド

```bash
$ cargo build --release
```

最適化がかかる  
最適化をかけると Rust のコードは速くなるが、コンパイル時間が長くなる

build コマンドを実行すると Cargo.lock ファイルが作られる  
Cargo は Cargo.lock でアプリケーションの依存を解決する

## Cargo プロジェクトを作る方法

Cargo で新規プロジェクトを作る場合は cargo new コマンド

```bash
$ cargo new hello_world --bin
```

実行アプリケーションを作る場合は `--bin` を渡す
