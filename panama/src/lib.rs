use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};
pub struct Sender<T> {
  inner: Arc<Inner<T>>,
}

impl<T> Clone for Sender<T> {
  fn clone(&self) -> Self {
    Sender {
      inner: Arc::clone(&self.inner),
    }
  }
}

impl<T> Sender<T> {
  pub fn send(&mut self, t: T) {
    let mut queue = self.inner.queue.lock().unwrap();

    queue.push_back(t);

    drop(queue);

    self.inner.available.notify_one();
  }
}

pub struct Receiver<T> {
  inner: Arc<Inner<T>>,
}

impl<T> Receiver<T> {
  pub fn receive(&mut self) -> T {
    let mut queue = self.inner.queue.lock().unwrap();
    loop {
      match queue.pop_front() {
        Some(t) => return t,
        None => queue = self.inner.available.wait(queue).unwrap(),
      }
    }
  }
}

struct Inner<T> {
  queue: Mutex<VecDeque<T>>,
  available: Condvar,
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
  let inner = Inner {
    queue: Mutex::default(),
    available: Condvar::default(),
  };
  let inner = Arc::new(inner);
  (
    Sender {
      inner: Arc::clone(&inner),
    },
    Receiver {
      inner: Arc::clone(&inner),
    },
  )
}

#[cfg(test)]
mod tests {
  use super::channel;

  #[test]
  fn ping_pong() {
    let (mut tx, mut rx) = channel();

    tx.send(42);
    assert_eq!(rx.receive(), 42);
  }
}
