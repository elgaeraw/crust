use crate::Sorter;

#[allow(dead_code)]
pub struct Quick;

#[allow(dead_code)]
pub fn quick<T: Ord>(slice: &mut [T]) {
  match slice.len() {
    0 | 1 => return,
    2 if slice[0] > slice[1] => {
      slice.swap(0, 1);
      return;
    }
    _ => {}
  }

  let (pivot, rest) = slice.split_first_mut().expect("slice is non-empty");

  let mut left = 0;
  let mut right = rest.len() - 1;

  while left <= right {
    if &rest[left] <= pivot {
      left += 1;
    } else if &rest[right] > pivot {
      if right == 0 {
        break;
      }
      right -= 1;
    } else {
      rest.swap(left, right);
      if right == 0 {
        break;
      }
      right -= 1;
    }
  }

  if left > right {
    slice.swap(0, left);
  }

  let (left, right) = slice.split_at_mut(left + 1);

  quick(left);
  quick(right);
}

impl Sorter for Quick {
  fn sort<T>(&self, slice: &mut [T])
  where
    T: Ord,
  {
    quick(slice);
  }
}

#[cfg(test)]
mod tests {
  use crate::Sorter;
  use crate::quick::Quick;

  #[test]
  fn qck_works() {
    let mut things = vec![10, 7, 8, 9, 5, 1];
    Quick.sort(&mut things);
    assert_eq!(things, &[1, 5, 7, 8, 9, 10]);

    let mut things = vec![4, 2, 3, 5, 1];
    Quick.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4, 5]);
  }
}
