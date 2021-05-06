use std::collections::HashMap;


pub fn map_test() {
  let mut hm: HashMap<&str, i32> = HashMap::new();
  hm.insert("张三", 1);
  hm.insert("张无忌", 2);
  hm.insert("郭靖", 3);

  println!("{:?}", &hm);

  for (_, val) in hm.iter_mut(){
    *val += 20;
  }
  println!("{:?}", hm);
}


pub fn find_non_repeated_substr(s: &str) ->usize {
  let mut last_occured: HashMap<char, usize> = HashMap::new();
  let mut start = 0;
  let mut max_length = 0;
  for (idx, val) in s.char_indices() {
    if last_occured.get(&val) != None && *(last_occured.get(&val).unwrap()) >= start {
      start = *(last_occured.get(&val).unwrap()) + 1;
    }
    if idx - start +1 > max_length {
      max_length = idx - start +1;
    }
    last_occured.insert(val, idx);
  }
  println!("{:?}", last_occured);
  return max_length;
}