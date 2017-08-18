# Environment

## Install

### CMake

#### On Windows

https://cmake.org/download/  
インストーラを使用してインストールする

`cmake`コマンドが使えるか確認する

### Emscripten

```
$ git clone https://github.com/juj/emsdk.git
$ cd emsdk
$ ./emsdk install sdk-incoming-64bit binaryen-master-64bit
$ ./emsdk activate sdk-incoming-64bit binaryen-master-64bit
```

#### On Windows

`emsdk_env.bat`を実行すると環境変数に`emcc`の実行パスが設定される
