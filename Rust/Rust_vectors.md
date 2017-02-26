# ベクター

`vector` は動的で拡張可能な配列で標準ライブラリの `Vec<T>` で実装されてる  
`T` は `vector` に持たせる任意の型を指定する  
`Vectors` は常にヒープに確保される  
以下のように `vec!` マクロで作ることができる

```rust
fn main() {
  let v = vec![1, 2, 3, 4, 5]; // v: Vec<i32>
  let v = vec!(1, 2, 3, 4, 5); // これでもいい
}
```

`vec!` マクロは `println!` とは違って `[]`(ブラケット)を使う  
`()` も使うことができるが `[]` を使うことが慣例となってる

`vec!` マクロの別の使い方として初期値を設定することができる

```rust
fn main() {
  let v = vec![0; 10];
}
```

`Vectors` はヒープ上に連続した `T` の配列として格納する  
そのため、コンパイル時に `T` のサイズがわかってないといけない  
コンパイル時にサイズがわからない場合にはポインタを格納するがそれ用に `Box` という型が用意されている

# 要素へのアクセス

`Vectors` の特定の値を取得する場合は `[]` を使う

```rust
fn main() {
  let v = vec![1, 2, 3, 4, 5];
  println!("The third element of v is {}", v[2]);
}
```

インデックス値は `0` から始め、`usize` 型でなければならない

```rust
fn main() {
  let v = vec![1, 2, 3, 4, 5];
  
  let i: usize = 0;
  let j: i32 = 0;
  
  // OK
  v[i];
  
  // コンパイルエラー
  v[j];
}
```

# 範囲外へのアクセス

以下のように範囲外の要素にアクセスすると `panic` を引き起こす
```rust
fn main() {
  let v = vec![1, 2, 3, 4, 5];
  println!("{}", v[7]);
}
```

範囲外の要素にアクセスした時にエラーハンドリングしたい場合は、`get` や `get_mut` を使うと `Option` でラップした値が返ってくるので、`match` 文を使うことで正常系と異常系の処理を安全かつ明瞭にかきわけられる

```rust
fn main() {
  let v = vec![1, 2, 3, 4, 5];
  
  match v.get(7) {
    Some(x) => println!("Item 7 is {}", x),
    None => println!("Thie vector is too short")
  }
}
```

# 繰り返し

`Vectors` のイテレーションには以下の3種類がある

```rust
fn main() {
  // 1
  let mut v = vec![1, 2, 3];
  for i in &v {
    println!("{} に対する参照。", i);
  }
  
  // 2
  let mut v = vec![1, 2, 3];
  for i in &mut v {
    println!("{} に対するミュータブルな参照。", i);
  }
  
  // 3
  let mut v = vec![1, 2, 3];
  for i in v {
    println!("vectorとその要素 {} のownershipを取得。", i);
  }
}
```

3番目の `ownership` を取得する形でのイテレーションを複数回行うことはできない  
`ownership` を取得しなければ何度も参照を取れる
