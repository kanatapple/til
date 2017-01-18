# ライブラリ

Rust が全てのプログラムでデフォルトで読み込むものは少しだけ  
-> これを prelude(プレリュード) という

prelude にないものは直接 `use` する必要がある

```rs
use std::io;
```

2つ目の prelude、io prelude もあって、io に関連する機能が提供される

