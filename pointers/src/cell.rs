use std::cell::UnsafeCell;

pub struct Cell<T> {
  value: UnsafeCell<T>,
}

impl<T> Cell<T> {
  pub fn new(val: T) -> Self {
    Cell {
      value: UnsafeCell::new(val),
    }
  }

  pub fn set(&self, val: T) {
    unsafe { *self.value.get() = val };
  }

  pub fn get(&self) -> T
  where
    T: Copy,
  {
    unsafe { *self.value.get() }
  }
}

#[cfg(test)]
mod failing_tests {

  use super::Cell;
  unsafe impl<T> Sync for Cell<T> {}

  impl<T> Cell<T> {
    pub fn get_ref(&self) -> &T {
      unsafe { &*self.value.get() }
    }
  }

  #[test]
  #[should_panic(expected = "assertion")]
  fn bad() {
    use std::sync::Arc;

    let x = Arc::new(Cell::new(0));
    let x1: Arc<Cell<i32>> = Arc::clone(&x);
    let jh1 = std::thread::spawn(move || {
      for _ in 0..100000 {
        x1.set(x1.get() + 1);
      }
    });
    let x2 = Arc::clone(&x);
    let jh2 = std::thread::spawn(move || {
      for _ in 0..100000 {
        x2.set(x2.get() + 1);
      }
    });

    jh1.join().unwrap();
    jh2.join().unwrap();

    assert_eq!(x.get(), 200000);
  }

  #[test]
  #[should_panic(expected = "assertion")]
  fn bad2() {
    let x = Cell::new(vec![42]);
    let first = &x.get_ref()[0];
    x.set(vec![]);
    assert_eq!(first, &42);
  }
}
