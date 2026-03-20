#![allow(clippy::vec_init_then_push)]

#[macro_export]
macro_rules! vecmac {
  () => {
    Vec::new()
  };
  ($($elem:expr),+) => {{
    let mut vs = Vec::new();
    $(
      vs.push($elem);
    )+
    vs
  }};
}

#[test]
fn empty_test() {
  let x: Vec<u32> = vecmac![];
  assert!(x.is_empty());
  format_args!("a");
}

#[test]
fn single_test() {
  let x: Vec<u32> = vecmac![42];
  assert_eq!(x.len(), 1);
  assert_eq!(x[0], 42);
}

#[test]
fn doublt_test() {
  let x: Vec<u32> = vecmac![42, 43];
  assert_eq!(x.len(), 2);
  assert_eq!(x[0], 42);
  assert_eq!(x[1], 43);
}
