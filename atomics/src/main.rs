#![allow(clippy::missing_spin_loop)]

use std::{
  cell::UnsafeCell,
  sync::atomic::{AtomicBool, Ordering},
  thread::spawn,
};

const LOCKED: bool = true;
const UNLOCKED: bool = false;

pub struct Mutex<T> {
  v: UnsafeCell<T>,
  locked: AtomicBool,
}

unsafe impl<T> Sync for Mutex<T> where T: Send {}

impl<T> Mutex<T> {
  pub fn new(t: T) -> Self {
    Self {
      locked: AtomicBool::new(UNLOCKED),
      v: UnsafeCell::new(t),
    }
  }
  pub fn with_lock<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
    while self
      .locked
      .compare_exchange_weak(UNLOCKED, LOCKED, Ordering::Relaxed, Ordering::Relaxed)
      .is_err()
    {
      // MESI Protocol
      while self.locked.load(Ordering::Relaxed) == LOCKED {}
    }
    let ret = f(unsafe { &mut *self.v.get() });
    self.locked.store(UNLOCKED, Ordering::Relaxed);
    ret
  }
}

fn main() {
  let l: &'static _ = Box::leak(Box::new(Mutex::new(0)));

  let handles: Vec<_> = (0..1000)
    .map(|_| {
      spawn(move || {
        let _: Vec<_> = (0..1000)
          .map(|_| {
            l.with_lock(|i| *i += 1);
          })
          .collect();
      })
    })
    .collect();

  for handle in handles {
    handle.join().unwrap();
  }

  assert_eq!(l.with_lock(|i| *i), 1000 * 1000);
}
