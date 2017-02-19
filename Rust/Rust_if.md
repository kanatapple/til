# if

動的型付け言語の `if` に近いもの

```rust
fn main() {
  let x = 5;
  if x == 5 {
    println!("x は 5 です");
  } else if x == 6 {
    println!("x は 6 です");
  } else {
    println!("x は 5 でも 6 でもありません");
  }
}
```

こういう書き方もできる

```rust
fn main() {
  let x = 5;
  let y = if x == 5 {
    10
  } else {
    15
  };
  // こういう書き方でもOK
  let y = if x == 5 { 10 } else { 15 };
}
```

これができるのは `if` が式だから  
その式の値は、選択された分岐中の最後の式の値になり、`else` のない `if` では、その値は常に `()` になる
