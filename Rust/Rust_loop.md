# ループ

Rust の繰り返し処理には `loop`、`while`、`for` の3種類がある

## loop

Rust で使えるループの中で最もシンプルな形式が無限 `loop`  
`loop` によって何らかの終了状態に到達するまでずっとループし続ける

```rust
fn main () {
  loop {
    println!("Loop forever!");
  }
}
```

## while

`while` ループは以下のように書く

```rust
fn main() {
  let mut x = 5;
  let mut done = false;
  while !done {
    x += x -3;
    
    println!("{}, x");
    
    if x % 5 == 0 {
      done = true;
    }
  }
}
```

何回ループするかわからない時、`while` ループは正しい選択

無限ループする時は以下のように書きたくなるが

```rust
fn main() {
  while true {
  }
}
```

`loop` を使う方が適している

```rust
fn main() {
  loop {
  }
}
```

Rust の制御フロー解析では、必ずループすると知っていることから、これを `while true` とは異なる構造として扱う  
一般にコンパイラへ与える情報量が多いほど、安全性が高くより良いコード生成に繋がるため、無限にループするつもりなら常に `loop` を使うべき

## for

特定の回数だけループする時は `for` を使う  
しかし、Rust の `for` ループは他のシステムプログラミング言語とは少し異なる働きをする

```rust
fn main() {
  for x in 0..10 {
    println!("{}", x);
  }
}
```

もう少し抽象的な用語を使うと以下のようになる

```rust
fn main() {
  for var in expression {
    code
  }
}
```

式(expression)は `IntoIterator` を用いてイテレータへと変換可能なアイテム  
イテレータは要素の連なりを返し、それぞれの要素がループ1回の反復になる  
その値は名前 `var` に束縛されループ本体で有効になり、ループ本体を抜けると次の要素がイテレータから取り出され、次のループ処理を行う  
それ以上の要素がない場合は `for` ループは終了する

上記の `0..10` が開始位置と終了位置を取る式で、同範囲の値を返すイテレータを与える  
上界はその値自身を含まないので 0 から 9 までの値を表示する

## 列挙

ループ中で何回目の繰り返しかを知る必要がある場合は `.enumerate()` 関数が使える

```rust
fn main() {
  // レンジを括弧で囲うのを忘れずに
  for (i, j) in (5..10).enumerate() {
    println!("i = {} and j = {}", i, j);
  }
}
```

イテレータを対象にした例

```rust
fn main() {
  let lines = vec!["one", "two", "three", "for"];
  for (linenumber, line) in lines.iter().enumerate() {
    println!("{}: {}", linenumber, line);
  }
}
```

## 反復の早期終了

2つのキーワード、`break` と `continue` がある

```rust
fn main () {
  let mut x = 5;
  loop {
    x += x - 3;
    println!("{}", x);
    if x % 5 == 0 { break; }
  }
}
```

無限ループの場合は、`loop` キーワードを使用して `break` で抜ける方がよい  
明示的に `return` 文でも早期終了できる

`continue` はループを終了させるのではなく、次の反復へすすめる

```rust
fn main() {
  for x in 0..10 {
    if x % 2 == 0 { continue; }
    println!("{}", x);
  }
}
```

## ループラベル

入れ子のループがあり、`break` や `continue` 文がどのループに対応するか指定する必要がある場合に使う  
デフォルトでは `break` や `continue` は最内ループに適用される  
外側のループに `break` や `continue` を使いたい場合は適用先を指定するラベルが使える

```rust
fn main() {
  'outer: for x in 0..10 {
    'inner: for y in 0..10 {
      if x % 2 == 0 { continue 'outer; }
      if y % 2 == 0 { continue 'inner; }
      println!("x: {}, y: {}", x, y);
    }
  }
}
```
