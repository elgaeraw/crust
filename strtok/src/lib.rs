pub fn strtok<'a>(s: &mut &'a str, delimiter: char) -> &'a str {
  if let Some(i) = s.find(delimiter) {
    let prefix = &s[..i];
    let suffix = &s[(i + delimiter.len_utf8())..];
    *s = suffix;
    prefix
  } else {
    let prefix = *s;
    *s = "";
    prefix
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let mut x = "hello world!";
    let hello = strtok(&mut x, ' ');
    assert_eq!(hello, "hello");
    assert_eq!(x, "world!");
  }

  #[test]
  fn foo_does_not_work() {
    fn foo<'a>(s: &mut &'a str, x: &'a str) {
      *s = x;
    }

    let mut x = "hello world!";
    let z = String::from("new world!");
    foo(&mut x, &z);
    // drop(z); // This will break if uncommented
    assert_eq!(x, "new world!");
  }
}
