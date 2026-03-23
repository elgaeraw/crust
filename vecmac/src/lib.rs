#![allow(clippy::vec_init_then_push, dead_code, non_local_definitions)]

#[macro_export]
macro_rules! vecmac {
  ($($elem:expr),*) => {{
    // let count = todo!();
    #[allow(unused_mut)]
    let mut vs = Vec::with_capacity($crate::vecmac![@COUNT; $($elem),*]);
    $(
      vs.push($elem);
    )*
    vs
  }};
  ($($elem:expr,)*) => {{
    $crate::vecmac![$($elem),*]
  }};
  ($elem: expr; $count: expr) => {{
    let count = $count;
    let mut vs = Vec::with_capacity(count);
    // let x = $elem;
    // for _ in 0..$count {
      // vs.push(x.clone());
    // }
    vs.extend(std::iter::repeat($elem).take(count));
    vs
  }};

  (@COUNT; $($elem: expr),*) => {
    <[()]>::len(&[$($crate::vecmac![@SUBST; $elem]),*])
  };

  (@SUBST; $elem: expr) => { () }
}

trait MaxValue {
  fn get_max_value() -> Self;
}

#[macro_export]
macro_rules! impl_max_value {
  ($t:ty, $prop:ident) => {{
    impl $crate::MaxValue for $t {
      fn get_max_value() -> Self {
        <$t>::$prop
      }
    }
  }};
}

#[test]
fn empty_test() {
  let x: Vec<u32> = vecmac![];
  assert!(x.is_empty());
  // format!("a {} {} {}", 1, 2, 3,);
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

#[test]
fn trailing() {
  let x: Vec<u32> = vecmac![
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
    27, 28, 29, 30,
  ];
  assert_eq!(x.len(), 30);
  assert_eq!(x[0], 1);
  assert_eq!(x[29], 30);
}

#[test]
fn count_test() {
  let x: Vec<u32> = vecmac![5; 20];
  assert_eq!(x.len(), 20);
  assert_eq!(x[0], 5);
  assert_eq!(x[19], 5);
}

#[test]
fn take_test() {
  let mut y = Some(5);
  let x: Vec<u32> = vecmac![y.take().unwrap(); 20];
  assert_eq!(x.len(), 20);
  assert_eq!(x[0], 5);
  assert_eq!(x[19], 5);
}

#[test]
fn impl_test() {
  impl_max_value!(u32, MAX);
  impl_max_value!(i32, MAX);
  impl_max_value!(i64, MAX);
  impl_max_value!(u64, MAX);

  assert_eq!(u32::get_max_value(), u32::MAX);
  assert_eq!(i32::get_max_value(), i32::MAX);
  assert_eq!(u64::get_max_value(), u64::MAX);
  assert_eq!(i64::get_max_value(), i64::MAX);
}

/// ```compile_fail
/// let x: Vec<u32> = vecmac::vecmac![5, "foo"];
/// ```
struct CompileFailTest;
