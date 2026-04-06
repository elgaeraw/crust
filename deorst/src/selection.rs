use crate::Sorter;

#[allow(dead_code)]
pub struct Selection;

impl Sorter for Selection {
  fn sort<T>(&self, slice: &mut [T])
  where
    T: Ord,
  {
    for unsorted in 0..slice.len() {
      // Manual Way
      // let mut smallest_in_rest = unsorted;
      // for i in (unsorted + 1)..slice.len() {
      //   if slice[i] < slice[smallest_in_rest] {
      //     smallest_in_rest = i;
      //   }
      // }

      // Iterator Way
      let smallest_in_rest = slice[unsorted..]
        .iter()
        .enumerate()
        .min_by_key(|&(_, v)| v)
        .map(|(i, _)| unsorted + i)
        .expect("slice is non-empty");

      if smallest_in_rest != unsorted {
        slice.swap(smallest_in_rest, unsorted);
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::Sorter;
  use crate::selection::Selection;

  #[test]
  fn sel_works() {
    let mut things = vec![4, 2, 3, 5, 1];
    Selection.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4, 5]);
  }
}
