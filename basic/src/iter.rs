pub fn iter_next_consumer() {
  let vec = vec![1, 2, 3, 4];
  let mut iter = vec.iter();
  println!("{:?}", iter.next());
  println!("{:?}", iter.next());
  println!("{:?}", iter.next());
  println!("{:?}", iter.next());
  println!("{:?}", iter.next());
  println!("{:?}", iter.next());
}