# 🦀 Rust Practical Cheat Sheet + Systems Learning Guide

---

# 🧠 CORE RULE (MOST IMPORTANT)

If method takes:

* `self` → **moves ownership**
* `&self` → **immutable borrow**
* `&mut self` → **mutable borrow**

---

# 🔥 OPTION<T>

```rust
as_ref()        // &Option<T> -> Option<&T>         // borrow inner value without moving
as_mut()        // &mut Option<T> -> Option<&mut T> // mutably borrow inner value without moving
unwrap()        // Option<T> -> T                   // panic if None
expect(msg)     // Option<T> -> T                   // panic with message
map(f)          // Option<T> -> Option<U>           // consumes Option, transforms value
and_then(f)     // Option<T> -> Option<U>           // consumes Option, chains fallible ops
ok_or(err)      // Option<T> -> Result<T,E>         // convert Option to Result
ok_or_else(f)   // Option<T> -> Result<T,E>         // lazy error creation
is_some()       // Option<T> -> bool                // check if Some
is_none()       // Option<T> -> bool                // check if None

take()          // &mut Option<T> -> Option<T>      // moves value out, leaves None
replace(val)    // &mut Option<T> -> Option<T>      // replaces value, returns old
```

---

# 🔥 RESULT<T, E>

```rust
unwrap()        // Result<T,E> -> T                 // panic if Err
expect(msg)     // Result<T,E> -> T                 // panic with message
map(f)          // Result<T,E> -> Result<U,E>       // transform Ok
map_err(f)      // Result<T,E> -> Result<T,F>       // transform Err
and_then(f)     // Result<T,E> -> Result<U,E>       // chain fallible ops
ok()            // Result<T,E> -> Option<T>         // drop error
err()           // Result<T,E> -> Option<E>         // extract error
?               // Result<T,E> -> T                 // early return on Err
```

---

# 🔥 ITERATORS

```rust
iter()          // Vec<T> -> Iter<&T>               // borrow items
iter_mut()      // Vec<T> -> Iter<&mut T>           // mutable borrow items
into_iter()     // Vec<T> -> IntoIter<T>            // move items

map(f)          // Iterator<T> -> Iterator<U>       // transform elements (lazy)
filter(f)       // Iterator<T> -> Iterator<T>       // filter elements (lazy)
find(f)         // Iterator<T> -> Option<T>         // first match (consumes)
collect()       // Iterator<T> -> Collection        // execute iterator
for_each(f)     // Iterator<T> -> ()                // consume with side effects
fold(init, f)   // Iterator<T> -> Acc               // reduce to one value

enumerate()     // Iterator<T> -> (usize, T)        // index + value
zip(other)      // Iterator<T> -> (T, U)            // pair iterators
take(n)         // Iterator<T> -> Iterator<T>       // first n items
skip(n)         // Iterator<T> -> Iterator<T>       // skip n items
flatten()       // Iterator<Option<T>> -> T         // remove nesting
```

---

# 🔥 OWNERSHIP HELPERS

```rust
as_ref()        // T -> &T                          // borrow without moving
as_mut()        // T -> &mut T                      // mutable borrow
clone()         // &T -> T                          // explicit deep copy (if Clone)
to_owned()      // &T -> T                          // convert borrowed -> owned
```

---

# 🔥 VEC<T>

```rust
new()           // () -> Vec<T>                     // create empty vector
push(val)       // Vec<T> -> ()                     // add element
pop()           // Vec<T> -> Option<T>              // remove last element
get(i)          // Vec<T> -> Option<&T>             // safe access
get_mut(i)      // Vec<T> -> Option<&mut T>         // mutable access
len()           // Vec<T> -> usize                  // length
is_empty()      // Vec<T> -> bool                   // check empty
```

---

# 🔥 HASHMAP<K, V>

```rust
new()           // () -> HashMap<K,V>               // create map
insert(k, v)    // -> Option<V>                     // insert, return old
get(k)          // -> Option<&V>                    // immutable access
get_mut(k)      // -> Option<&mut V>                // mutable access
remove(k)       // -> Option<V>                     // remove key

entry(k)        // -> Entry<K,V>                    // advanced API
```

## Entry API

```rust
or_insert(v)        // Entry -> &mut V              // insert if missing
or_insert_with(f)   // Entry -> &mut V              // lazy insert
and_modify(f)       // Entry -> Entry               // modify existing value
```

---

# 🔥 STRINGS

```rust
to_string()     // &str -> String                  // allocate new string
to_owned()      // &str -> String                  // idiomatic conversion
as_str()        // String -> &str                  // borrow slice
push(c)         // String -> ()                    // add char
push_str(s)     // String -> ()                    // add string
```

---

# 🔥 SMART POINTERS

```rust
Box::new(x)         // T -> Box<T>                 // heap allocation

Rc::new(x)          // T -> Rc<T>                  // shared ownership (single-thread)
Rc::clone(&x)       // Rc<T> -> Rc<T>              // increment ref count

Arc::new(x)         // T -> Arc<T>                 // thread-safe shared ownership
Arc::clone(&x)      // Arc<T> -> Arc<T>            // increment atomic count
```

---

# 🔥 INTERIOR MUTABILITY

```rust
RefCell::new(x)     // T -> RefCell<T>             // runtime borrow checking
borrow()            // RefCell<T> -> Ref<T>        // immutable borrow
borrow_mut()        // RefCell<T> -> RefMut<T>     // mutable borrow
```

---

# 🔥 CONCURRENCY

```rust
Mutex::new(x)       // T -> Mutex<T>               // mutual exclusion
lock()              // Mutex<T> -> MutexGuard<T>   // acquire lock

RwLock::new(x)      // T -> RwLock<T>              // multiple readers
read()              // -> RwLockReadGuard<T>       // shared access
write()             // -> RwLockWriteGuard<T>      // exclusive access
```

---

# 🔥 PATTERN MATCHING

```rust
if let Some(x) = opt {}            // handle only Some
while let Some(x) = iter.next() {} // manual iteration
matches!(x, pattern)              // boolean pattern check
```

---

# 🔥 CONVERSIONS

```rust
into()         // T -> U (via Into trait)         // implicit conversion
from(x)        // T -> U (via From trait)         // explicit conversion
```

---

# 🧠 SYSTEMS LEARNING TRACK (BUILD THESE IN RUST)

---

## 1. JOB QUEUE SYSTEM

Architecture:

```
Producer -> Queue -> Workers -> Results
```

Learn:

* Mutex vs channels
* thread pools
* retry logic
* backpressure

---

## 2. THREAD POOL

Architecture:

```
Tasks -> Queue -> Worker Threads
```

Learn:

* concurrency primitives
* scheduling
* graceful shutdown

---

## 3. HTTP SERVER

Architecture:

```
Socket -> Accept -> Parse -> Route -> Respond
```

Learn:

* networking
* async vs threads
* request lifecycle

---

## 4. IN-MEMORY CACHE (REDIS-LIKE)

Architecture:

```
HashMap + Concurrency Control
```

Learn:

* RwLock vs Mutex
* read-heavy optimization
* eviction strategies

---

# 🧠 FINAL NOTES

* Prefer `as_ref()` / `as_mut()` to avoid moves
* Use `take()` when you want ownership out
* Use `entry()` instead of manual checks in HashMap
* Iterators are lazy — nothing happens until consumed

---

# 🚀 HOW TO USE THIS

* Keep this open while coding
* Do NOT memorize — recognize patterns
* Apply immediately in projects

---

# 🧠 COMPILER ERRORS → FIX PATTERNS (HIGH VALUE)

These are the errors you will see daily. Learn these → you become fast in Rust.

---

## ❌ error: use of moved value

```rust
let s = String::from("hi");
let t = s;
println!("{}", s); // error
```

### 🔧 Fix:

```rust
let t = s.clone();        // explicit copy
// OR
let t = &s;               // borrow instead of move
```

👉 Reason: ownership moved when assigning

---

## ❌ error: cannot borrow as mutable

```rust
let x = 5;
let y = &mut x; // error
```

### 🔧 Fix:

```rust
let mut x = 5;
let y = &mut x;
```

👉 Reason: variable must be declared `mut`

---

## ❌ error: cannot borrow immutable reference as mutable

```rust
let x = &5;
*x += 1; // error
```

### 🔧 Fix:

```rust
let mut v = 5;
let x = &mut v;
*x += 1;
```

👉 Reason: need mutable reference `&mut`

---

## ❌ error: value borrowed here after move

```rust
let v = vec![1,2,3];
for x in v {
    println!("{}", x);
}
println!("{:?}", v); // error
```

### 🔧 Fix:

```rust
for x in &v {          // borrow instead
    println!("{}", x);
}
println!("{:?}", v);
```

👉 Reason: `into_iter()` moved the vector

---

## ❌ error: cannot move out of borrowed content

```rust
let opt = Some(String::from("hi"));
let x = opt.unwrap(); // moves
println!("{:?}", opt); // error
```

### 🔧 Fix:

```rust
let x = opt.as_ref().unwrap(); // borrow instead
```

👉 OR

```rust
let x = opt.take(); // move safely, leaves None
```

---

## ❌ error: multiple mutable borrows

```rust
let mut v = vec![1,2,3];
let a = &mut v;
let b = &mut v; // error
```

### 🔧 Fix:

```rust
let a = &mut v;
// use a
let b = &mut v;
```

👉 OR use scope blocks

---

## ❌ error: cannot borrow as mutable more than once (HashMap)

```rust
let mut map = HashMap::new();
let v1 = map.get_mut("a");
let v2 = map.get_mut("a"); // error
```

### 🔧 Fix:

```rust
map.entry("a").and_modify(|v| *v += 1);
```

👉 Use `entry()` API instead

---

## ❌ error: temporary value dropped while borrowed

```rust
let r = &String::from("hi");
println!("{}", r);
```

### 🔧 Fix:

```rust
let s = String::from("hi");
let r = &s;
```

👉 Reason: temporary gets dropped too early

---

## ❌ error: mismatched types (Option vs value)

```rust
let x: Option<i32> = Some(5);
let y = x + 1; // error
```

### 🔧 Fix:

```rust
let y = x.map(|v| v + 1);
```

👉 OR

```rust
let y = x.unwrap_or(0) + 1;
```

---

## ❌ error: expected &T found T

```rust
fn foo(x: &i32) {}
foo(5); // error
```

### 🔧 Fix:

```rust
foo(&5);
```

---

## ❌ error: expected T found &T

```rust
fn foo(x: i32) {}
let v = 5;
foo(&v); // error
```

### 🔧 Fix:

```rust
foo(v);
```

---

## ❌ error: borrow does not live long enough

```rust
let r;
{
    let x = 5;
    r = &x;
}
println!("{}", r); // error
```

### 🔧 Fix:

```rust
let x = 5;
let r = &x;
println!("{}", r);
```

👉 Reason: reference outlives value

---

# 🧠 FINAL TAKEAWAY

* Most errors = ownership or borrowing mistakes
* Fix by choosing: move, borrow, or clone
* Compiler is not your enemy — it's your guide

---
