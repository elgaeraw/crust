use deorst::*;
use rand::RngExt;
use std::cell::Cell;
use std::rc::Rc;

#[derive(Clone, Debug)]
struct SortEvaluator<T> {
  t: T,
  cmps: Rc<Cell<usize>>,
}

impl<T: PartialEq> PartialEq for SortEvaluator<T> {
  fn eq(&self, other: &Self) -> bool {
    self.t == other.t
  }
}

impl<T: Eq> Eq for SortEvaluator<T> {}

impl<T: PartialOrd> PartialOrd for SortEvaluator<T> {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    self.cmps.set(self.cmps.get() + 1);
    self.t.partial_cmp(&other.t)
  }
}

impl<T: Ord> Ord for SortEvaluator<T> {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    self.partial_cmp(other).expect("T: Ord")
  }
}

fn bench<S, T>(sorter: S, values: &[SortEvaluator<T>], counter: Rc<Cell<usize>>) -> usize
where
  S: Sorter,
  T: Ord + Clone,
{
  let mut values = values.to_vec();
  counter.set(0);
  sorter.sort(&mut values);
  assert!(values.is_sorted());
  counter.get()
}

fn main() {
  println!("Algorithm n Comparisons");
  let mut rng = rand::rng();
  let counter = Rc::new(Cell::new(0));
  for n in [0, 1, 10, 100, 1000, 10000, 100000] {
    for _ in 0..10 {
      let values: Vec<SortEvaluator<usize>> = (0..n)
        .map(|_| SortEvaluator {
          t: rng.random_range(0..usize::MAX),
          cmps: Rc::clone(&counter),
        })
        .collect();

      let took_bbl = bench(Bubble, &values, Rc::clone(&counter));
      println!("Bubble {} {}", n, took_bbl);

      let took_ins_smt = bench(Insertion { smart: true }, &values, Rc::clone(&counter));
      println!("InsertionSmart {} {}", n, took_ins_smt);

      let took_ins_dmb = bench(Insertion { smart: false }, &values, Rc::clone(&counter));
      println!("InsertionDumb {} {}", n, took_ins_dmb);

      let took_sel = bench(Selection, &values, Rc::clone(&counter));
      println!("Selection {} {}", n, took_sel);

      let took_qck = bench(Quick, &values, Rc::clone(&counter));
      println!("Quick {} {}", n, took_qck);
    }
  }
}
