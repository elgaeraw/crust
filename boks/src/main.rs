// #![feature(dropck_eyepatch)]
#![allow(unused_variables, unused_assignments)]

use std::ptr::NonNull;
use std::{fmt::Debug, marker::PhantomData};

pub struct Boks<T> {
  p: NonNull<T>,
  _t: PhantomData<T>,
}

impl<T> Boks<T> {
  pub fn ny(t: T) -> Self {
    Boks {
      p: unsafe { NonNull::new_unchecked(Box::into_raw(Box::new(t))) },
      _t: PhantomData,
    }
  }
}

// unsafe impl<#[may_dangle] T> Drop for Boks<T> {
impl<T> Drop for Boks<T> {
  fn drop(&mut self) {
    drop(unsafe { Box::from_raw(self.p.as_mut()) });
  }
}

impl<T> std::ops::Deref for Boks<T> {
  type Target = T;
  fn deref(&self) -> &Self::Target {
    unsafe { self.p.as_ref() }
  }
}

impl<T> std::ops::DerefMut for Boks<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    unsafe { &mut *self.p.as_mut() }
  }
}

#[derive(Debug)]
pub struct Oisann<T: Debug>(T);

impl<T: Debug> Drop for Oisann<T> {
  fn drop(&mut self) {
    println!("Dropping {:#?}", self.0);
  }
}

fn main() {
  let x = 42;
  let b = Boks::ny(x);
  println!("*b = {:#?}", *b);
  println!("x = {:#?}", x);

  let mut y = 42;
  let b = Boks::ny(&mut y);
  // let b = Box::new(&mut y);
  println!("*b = {:#?}", *b);

  // drop(b);
  // println!("y = {:#?}", y);

  let /* mut */ z = 42;
  // let b = Boks::ny(Oisann(&mut z));
  // let b = Box::new(Oisann(&mut z));
  // println!("{:#?}", *b);
  println!("z = {:#?}", z);

  let s = String::from("hei");
  let mut box1 = Box::new(&*s);
  let box2: Box<&'static str> = Box::new("heisann");
  box1 = box2;

  let s = String::from("hei");
  let mut boks1 = Boks::ny(&*s);
  let boks2: Boks<&'static str> = Boks::ny("heisann");
  boks1 = boks2;

  let mut a = 42;
  let mut it = std::iter::empty();
  let mut o = Some(Oisann(&mut a));
  {
    o = it.next();
  }
  let _ = it.next();
  // drop(o);
  println!("a = {:#?}", a);
}
