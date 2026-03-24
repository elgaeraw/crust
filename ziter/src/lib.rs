pub struct Flatten<O>
where
  O: Iterator,
  O::Item: IntoIterator,
{
  outer: O,
  inner: Option<<O::Item as IntoIterator>::IntoIter>,
}

impl<O> Flatten<O>
where
  O: Iterator,
  O::Item: IntoIterator,
{
  fn new(iter: O) -> Self {
    Flatten {
      outer: iter,
      inner: None,
    }
  }
}

pub fn flatten<O>(iter: O) -> Flatten<O::IntoIter>
where
  O: IntoIterator,
  O::Item: IntoIterator,
{
  Flatten::new(iter.into_iter())
}

impl<O> Iterator for Flatten<O>
where
  O: Iterator,
  O::Item: IntoIterator,
{
  type Item = <O::Item as IntoIterator>::Item;

  fn next(&mut self) -> Option<Self::Item> {
    if let Some(ref mut inner_iter) = self.inner {
      if let Some(i) = inner_iter.next() {
        return Some(i);
      }
      self.inner = None;
      return self.next();
    }
    let inner = self.outer.next()?.into_iter();
    self.inner = Some(inner);

    self.next()
  }
}

#[test]
fn wide() {
  let v = vec![vec!["a", "b"], vec!["c"], vec![], vec!["d"]];

  for x in flatten(v) {
    println!("{}", x);
  }
}
