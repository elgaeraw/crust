pub trait IteratorExt: IntoIterator {
  fn flatten_ext(self) -> Flatten<Self::IntoIter>
  where
    Self: Sized,
    Self::Item: IntoIterator,
  {
    flatten(self)
  }
}

impl<T> IteratorExt for T
where
  T: IntoIterator,
{
  fn flatten_ext(self) -> Flatten<Self::IntoIter>
  where
    Self: Sized,
    Self::Item: IntoIterator,
  {
    flatten(self)
  }
}

pub struct Flatten<O>
where
  O: Iterator,
  O::Item: IntoIterator,
{
  outer: O,
  front_iter: Option<<O::Item as IntoIterator>::IntoIter>,
  back_iter: Option<<O::Item as IntoIterator>::IntoIter>,
}

impl<O> Flatten<O>
where
  O: Iterator,
  O::Item: IntoIterator,
{
  fn new(iter: O) -> Self {
    Flatten {
      outer: iter,
      front_iter: None,
      back_iter: None,
    }
  }
}

impl<O> Iterator for Flatten<O>
where
  O: Iterator,
  O::Item: IntoIterator,
{
  type Item = <O::Item as IntoIterator>::Item;

  fn next(&mut self) -> Option<Self::Item> {
    if let Some(ref mut front_iter) = self.front_iter {
      if let Some(i) = front_iter.next() {
        return Some(i);
      }
      self.front_iter = None;
      return self.next();
    }

    if let Some(next_inner) = self.outer.next() {
      self.front_iter = Some(next_inner.into_iter());
    } else {
      return self.back_iter.as_mut()?.next();
    }

    self.next()
  }
}

impl<O> DoubleEndedIterator for Flatten<O>
where
  O: DoubleEndedIterator,
  O::Item: IntoIterator,
  <O::Item as IntoIterator>::IntoIter: DoubleEndedIterator,
{
  fn next_back(&mut self) -> Option<Self::Item> {
    if let Some(ref mut back_iter) = self.back_iter {
      if let Some(i) = back_iter.next_back() {
        return Some(i);
      }
      self.back_iter = None;
      return self.next_back();
    }
    if let Some(next_back_inner) = self.outer.next_back() {
      self.back_iter = Some(next_back_inner.into_iter());
    } else {
      return self.front_iter.as_mut()?.next_back();
    }

    self.next_back()
  }
}

pub fn flatten<O>(iter: O) -> Flatten<O::IntoIter>
where
  O: IntoIterator,
  O::Item: IntoIterator,
{
  Flatten::new(iter.into_iter())
}

#[test]
fn wide() {
  let v = vec![vec!["a", "b"], vec!["c"], vec![], vec!["d"]];

  assert_eq!(flatten(v).collect::<Vec<_>>(), vec!["a", "b", "c", "d"]);
}

#[test]
fn wide_back() {
  let v = vec![vec!["a", "b"], vec!["c"], vec![], vec!["d"]];

  assert_eq!(
    flatten(v).rev().collect::<Vec<_>>(),
    vec!["d", "c", "b", "a"]
  );
}

#[test]
fn both_ends() {
  let mut iter = flatten(vec![
    vec!["a", "b"],
    vec!["c", "d", "e"],
    vec![],
    vec!["f", "g"],
  ]);

  assert_eq!(iter.next(), Some("a"));
  assert_eq!(iter.next_back(), Some("g"));
  assert_eq!(iter.next(), Some("b"));
  assert_eq!(iter.next_back(), Some("f"));
  assert_eq!(iter.next(), Some("c"));
  assert_eq!(iter.next_back(), Some("e"));
  assert_eq!(iter.next(), Some("d"));
  assert_eq!(iter.next_back(), None);
  assert_eq!(iter.next(), None);
  assert_eq!(iter.next_back(), None);
}

#[test]
fn ext() {
  let v = vec![vec!["a", "b"], vec!["c"], vec![], vec!["d"]];

  assert_eq!(
    v.flatten_ext().collect::<Vec<_>>(),
    vec!["a", "b", "c", "d"]
  );
}
