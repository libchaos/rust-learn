#[derive(Debug, Clone)]
struct Foo {
  x:i32,
  y:String,
}

pub fn OwnerShip(){
  let foo = Foo{x:9, y: String::from("hello")};

  let other = foo.clone();

  println!("{:?} {:?}", foo, other);
}