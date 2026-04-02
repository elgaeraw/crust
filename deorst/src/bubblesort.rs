use crate::Sorter;

#[allow(dead_code)]
pub struct BubbleSort;

impl Sorter for BubbleSort {
  fn sort<T>(slice: &mut [T])
  where
    T: Ord,
  {
    let mut swapped = true;

    while swapped {
      swapped = false;
      for i in 1..slice.len() {
        if slice[i - 1] > slice[i] {
          slice.swap(i - 1, i);
          swapped = true;
        }
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::bubblesort::BubbleSort;
  use crate::sort;

  #[test]
  fn bbl_works() {
    let mut things = vec![4, 2, 3, 5, 1];
    sort::<_, BubbleSort>(&mut things);
    assert_eq!(things, &[1, 2, 3, 4, 5]);
  }
}
