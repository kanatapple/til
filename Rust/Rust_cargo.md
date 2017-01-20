# Cargo

## init

既存のディレクトリに新規プロジェクトを作成できる  
.gitignore に不要なファイルを追記してくれてる？

```bash
$ cargo init [--bin]
```

プロジェクトの切り替えはできない・・？  
`cargo init` で作成した後に `cargo init --bin`とか

## update

Cargo.lock を無視して、指定したバージョンを満たす最新版を探す  
デフォルトではパッチバージョンの最新版を探さない  
マイナーバージョン、メジャーバージョンも含めて最新版を使いたい場合は Cargo.toml に記述する

## Cargo.toml

セクションに続く行は次のセクションが始まるまでそのセクションに属する

### [dependencies]セクション

どの外部クレートのどのバージョンに依存するかの情報を記述する

Cargo はセマンティックバージョンを理解する

```toml
[dependencies]
rand="0.3.0"
```

上記の場合、`^0.3.0` の略記で「0.3.0と互換性のあるもの」を意味する  
正確に `0.3.0` を使いたい場合は、`rand="=0.3.0"` と記述する  
最新版を使いたい場合は `*` を使う

Cargo はパッケージの最新版をレジストリ(Crates.ioのコピー)から取得する  
レジストリって %USERPROFILE%/.cargo/registry にあるもの？？  
レジストリをアップデートした後に [dependencies] を確認し、まだダウンロードしてないものをダウンロード

## Cargo.lock

プロジェクトを最初にビルドした時に、Cargo は基準を満たす全てのバージョンを検索し書き出す  
その後のビルドでは Cargo.lock ファイルがあるか確認し、バージョン検索せずに指定されたものを使う  
明示的にアップデートしない限り指定されたバージョンを使う