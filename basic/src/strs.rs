#[derive(Debug)]
pub struct StrSplit<'a> {
  remainder: &'a str,
  delimiter: &'a str,
}


impl<'a> StrSplit<'a> {
  pub fn new(haystack: &'a str, delimiter: &'a str) -> Self {
    Self {
      remainder:haystack,
      delimiter,
    }
  }
}

impl<'a> Iterator for StrSplit<'a> {
  type Item = &'a str;
  fn next(&mut self) -> Option<Self::Item> {
    if let Some(next_delim) = self.remainder.find(self.delimiter) {
      let util_delimiter = &self.remainder[..next_delim];
      self.remainder = &self.remainder[(next_delim +self.delimiter.len())..];
      Some(util_delimiter)
    } else if self.remainder.is_empty() {
      None
    } else {
      let rest = self.remainder;
      self.remainder = "";
      Some(rest)
    }
  }
}

// #[test]
// fn str_split(){
//   let hstack = "a b c d";
//   let delimiter = " ",
//   let letters = StrSplit(hstack, delimiter);
//   assert_eq!(letters, vec!["a", "b", "c", "d"].into_iter());
// }