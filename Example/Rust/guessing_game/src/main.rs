extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

// プログラムのエントリーポイント
// 戻り値の型を指定していないので、空のタプル () として扱われる
fn main() {
  println!("Guess the number!");

  // 現在いるスレッドにローカルな乱数生成器のコピーを取得し、乱数を生成
  let secret_number = rand::thread_rng().gen_range(1, 101);

  // println!("The secret number is: {}", secret_number);

  loop {
    println!("Please input your guess.");

    // ::new() 構文は特定の型の「関連関数」、もしくは「スタティックメソッド」と呼ぶ
    // ↓StringのインスタンスではなくString自体に関連付けられている関数
    let mut guess = String::new();

    // resultを使用しないと警告が出る
    let result = io::stdin().read_line(&mut guess);
    // println!("{:?}", result);

    result.expect("Failed to read line");

    // 新しい定義で隠す(シャドーイング)
    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_)  => {
        println!("Please type a number.");
        continue
      }
    };

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less    => println!("Too small!!"),
        Ordering::Greater => println!("Too big!!"),
        Ordering::Equal   => {
          println!("You Win!!");
          break; // ループを抜ける
        }
    }
  }
}
