use std::marker::PhantomData;
use std::ops::Deref;
use std::ptr::NonNull;

use crate::cell::Cell;

struct RcInner<T> {
  value: T,
  refcount: Cell<usize>,
}

pub struct Rc<T> {
  inner: NonNull<RcInner<T>>,
  _marker: PhantomData<RcInner<T>>,
}

impl<T> Rc<T> {
  pub fn new(val: T) -> Self {
    let inner = Box::new(RcInner {
      value: val,
      refcount: Cell::new(0),
    });
    Rc {
      inner: unsafe { NonNull::new_unchecked(Box::into_raw(inner)) },
      _marker: PhantomData,
    }
  }
}

impl<T> Deref for Rc<T> {
  type Target = T;
  fn deref(&self) -> &Self::Target {
    &unsafe { self.inner.as_ref() }.value
  }
}

impl<T> Clone for Rc<T> {
  fn clone(&self) -> Self {
    let inner = unsafe { self.inner.as_ref() };
    let c = inner.refcount.get();
    inner.refcount.set(c + 1);
    Rc {
      inner: self.inner,
      _marker: PhantomData,
    }
  }
}

impl<T> Drop for Rc<T> {
  fn drop(&mut self) {
    let inner = unsafe { self.inner.as_ref() };
    let c = inner.refcount.get();
    if c == 1 {
      let _ = unsafe { Box::from_raw(self.inner.as_ptr()) };
      // Below return generates a warning that this is not needed but that is only correct untill no-one
      // is using inner after if block. keeping this return here ensures that even if someone changes
      // this function, after dropping self.inner, we never execute anything and function just returns
      #[allow(clippy::needless_return)]
      return;
    } else {
      inner.refcount.set(c - 1);
    }
  }
}
