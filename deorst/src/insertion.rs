use crate::Sorter;

#[allow(dead_code)]
pub struct Insertion {
  smart: bool,
}

impl Sorter for Insertion {
  fn sort<T>(&self, slice: &mut [T])
  where
    T: Ord,
  {
    for unsorted in 1..slice.len() {
      if !self.smart {
        let mut i = unsorted;
        while i > 0 && slice[i - 1] > slice[i] {
          slice.swap(i - 1, i);
          i -= 1;
        }
      } else {
        let i = match slice[..unsorted].binary_search(&slice[unsorted]) {
          Ok(i) => i,
          Err(i) => i,
        };

        slice[i..=unsorted].rotate_right(1);
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::Sorter;
  use crate::insertion::Insertion;

  #[test]
  fn ins_works_dumb() {
    let mut things = vec![4, 2, 3, 5, 1];
    Insertion { smart: false }.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4, 5]);
  }

  #[test]
  fn ins_works_smart() {
    let mut things = vec![4, 2, 3, 5, 1];
    Insertion { smart: true }.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4, 5]);
  }
}
