//! Empty doc
// #![warn(missing_debug_implementations, missing_docs, rust_2018_idioms)]
#![warn(missing_debug_implementations, rust_2018_idioms)]

#[derive(Debug)]
pub struct StrSplit<'haystack, D> {
  haystack: Option<&'haystack str>,
  delimiter: D,
}

impl<'haystack, D> StrSplit<'haystack, D> {
  pub fn new(haystack: &'haystack str, delimiter: D) -> Self {
    Self {
      haystack: Some(haystack),
      delimiter,
    }
  }
}

pub trait Delimiter {
  fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}

impl<'haystack, D> Iterator for StrSplit<'haystack, D>
where
  D: Delimiter,
{
  type Item = &'haystack str;

  fn next(&mut self) -> Option<Self::Item> {
    let haystack = self.haystack.as_mut()?;
    if let Some((delim_start, delim_end)) = self.delimiter.find_next(haystack) {
      let until_delimiter = &haystack[..delim_start];
      *haystack = &haystack[delim_end..];
      Some(until_delimiter)
    } else {
      self.haystack.take()
    }
  }
}

impl Delimiter for &str {
  fn find_next(&self, s: &str) -> Option<(usize, usize)> {
    let delim_start = s.find(self)?;
    Some((delim_start, delim_start + self.len()))
  }
}

impl Delimiter for char {
  fn find_next(&self, s: &str) -> Option<(usize, usize)> {
    s.char_indices()
      .find(|(_, c)| c == self)
      .map(|(start, _)| (start, start + self.len_utf8()))
  }
}

impl<F> Delimiter for F
where
  F: Fn(char) -> bool,
{
  fn find_next(&self, s: &str) -> Option<(usize, usize)> {
    s.char_indices()
      .find(|(_, c)| self(*c))
      .map(|(start, c)| (start, start + c.len_utf8()))
  }
}

pub fn until_char(s: &str, c: char) -> &str {
  StrSplit::new(s, c)
    .next()
    .expect("StrSplit always returns at least one value")
}

#[test]
fn until_char_test() {
  assert_eq!(until_char("hello world!", 'o'), "hell");
}

#[test]
fn fn_split_numeric() {
  let haystack = "abc1def2ghi3";
  let letters = StrSplit::new(haystack, char::is_numeric);
  assert!(letters.eq(vec!["abc", "def", "ghi", ""]));
}

#[test]
fn fn_split_upper() {
  let haystack = "abcXdefYghiZ";
  let letters = StrSplit::new(haystack, char::is_uppercase);
  assert!(letters.eq(vec!["abc", "def", "ghi", ""]));
}

#[test]
fn it_works() {
  let haystack = "a b c d e f";
  let letters = StrSplit::new(haystack, " ");
  assert!(letters.eq(vec!["a", "b", "c", "d", "e", "f"]));
}
#[test]
fn tail() {
  let haystack = "a b c d e ";
  let letters = StrSplit::new(haystack, " ");
  assert!(letters.eq(vec!["a", "b", "c", "d", "e", ""]));
}
