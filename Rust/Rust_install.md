# インストール

## Windows

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