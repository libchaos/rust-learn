use std::collections::VecDeque;

#[warn(dead_code)]
pub fn is_parlindrome(s: &str) ->bool {
  let mut v = VecDeque::new();
  for ch in s.chars() {
    v.push_back(ch);
  }
  let mut flag = true;
  while v.len() > 1 && flag { 
    let first = v.pop_front();
    let last = v.pop_back();
    if first != last {
      flag = false;
    }
  }
  return flag;
  
}