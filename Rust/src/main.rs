struct Foo<'a> {
  x: &'a i32
}

fn main() {
  let x;

  {
    let y = &5;
    let f = Foo { x: y };
    x = &f.x;
  }

  println!("{}", x);
}
