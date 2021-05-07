pub fn clousure_test() {
  let add_one = |x| x + 1;

  println!("{}", add_one(2));
}


pub fn catch(){
  let i = 1;
  let add_i = |x| x + i;

  println!("add i is {}", add_i(7));
}