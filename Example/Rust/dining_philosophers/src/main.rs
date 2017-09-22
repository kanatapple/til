use std::thread;
use std::time::Duration;
use std::sync::{Mutex, Arc};

struct Philosopher {
  name: String,
  left: usize,
  right: usize
}

impl Philosopher {
  // selfを受け取らないので関連関数
  fn new(name: &str, left: usize, right: usize) -> Philosopher {
    Philosopher {
      name: name.to_string(),
      left: left,
      right: right
    }
  }

  // メソッドは明示的なselfパラメータを受け取る
  fn eat(&self, table: &Table) {
    // アンダースコアを使うと意図的に使わないということをRustに伝えることができる(警告が出ない)
    let _left = table.forks[self.left].lock().unwrap();
    // thread::sleep(Duration::from_millis(150));
    let _right = table.forks[self.right].lock().unwrap();

    println!("{} is eating.", self.name);

    thread::sleep(Duration::from_millis(1000));

    println!("{} is done eating.", self.name);
  }
}

struct Table {
  forks: Vec<Mutex<()>>
}

fn main() {
  let table = Arc::new(Table { forks: vec![
    Mutex::new(()),
    Mutex::new(()),
    Mutex::new(()),
    Mutex::new(()),
    Mutex::new(()),
  ]});

  let philosophers = vec![
    Philosopher::new("Judith Butler", 0, 1),
    Philosopher::new("Gilles Deleuze", 1, 2),
    Philosopher::new("Karl Marx", 2, 3),
    Philosopher::new("Emma Goldman", 3, 4),
    Philosopher::new("Michel Foucault", 0, 4)
  ];

  // _ はプレースホルダ。「handles は何らかの型のベクトルだが、その型が何であるかはRustが解決しろ」と指示している
  // スレッドに各要素を渡すため、イテレータに対して map を呼び出し、その引数として要素ごとに順番に呼ばれるクロージャを渡している
  let handles: Vec<_> = philosophers.into_iter().map(|p| {
    let table = table.clone();

    // 並行実行される部分
    // spawn はクロージャを1つ引数に取り、新しいスレッド上でそのクロージャを実行する
    // このクロージャは move という特別なアノテーションを必要とし、これによってキャプチャする値の所有権がクロージャ内へと移動する
    // ↓戻り値を返すのでセミコロンは付けない
    thread::spawn(move || {
      p.eat(&table);
    })
  }).collect();

  for h in handles {
    // join() を呼び出し、各スレッドの実行が完了するまで実行をブロックする
    h.join().unwrap();
  }
}
