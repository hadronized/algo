#[derive(Debug)]
struct Heap<T> {
  buf: Vec<HeapNode<T>>
}

impl<T> Heap<T> {
  // Create an empty heap.
  //
  // O(1)
  pub fn new() -> Self {
    Heap {
      buf: Vec::new()
    }
  }

  // Number of elements in the heap.
  //
  // O(1)
  pub fn len(&self) -> usize {
    self.buf.len()
  }

  // Compute the balance factor of a node identified by its index.
  fn balance_factor(&self, i: usize) -> isize {
    self.buf[i].right.map(|r| self.buf[r].height as isize).unwrap_or(0) -
      self.buf[i].left.map(|l| self.buf[l].height as isize).unwrap_or(0)
  }

  // Push a value at the end of the buffer and get a node index.
  //
  // O(1)
  fn push(&mut self, x: T, height: usize, left: Option<usize>, right: Option<usize>) -> usize {
    let i = self.buf.len();
    self.buf.push(HeapNode {
      value: x,
      height,
      left,
      right
    });
    i
  }

  // Insert something in the heap.
  //
  // O(log(N))
  pub fn insert(&mut self, x: T) where T: Ord {
    if self.buf.is_empty() {
      self.push(x, 0, None, None);
    } else {
      self.insert_(0, x);
    }
  }

  // Insert something in the heap.
  //
  // O(log(N))
  fn insert_(&mut self, i: usize, x: T) where T: Ord {
    if x <= self.buf[i].value {
      // we can insert in subtrees
      if let Some(lt) = self.buf[i].left {
        if let Some(rt) = self.buf[i].right {
          // both subtrees exist; insert in the one with the smallest balance factor
          if self.balance_factor(lt) <= self.balance_factor(rt) {
            self.insert_(lt, x);
          } else {
            self.insert_(rt, x);
          }
        } else {
          // no right node, insert here
          let ri = self.push(x, self.buf[i].height + 1, None, None);
          self.buf[i].right = Some(ri);
        }
      } else {
        // no left node, insert here
          let li = self.push(x, self.buf[i].height + 1, None, None);
          self.buf[i].left = Some(li);
      }
    } else {
      // we need to replace this node; find which subtree has the biggest balance factor
      if let Some(lt) = self.buf[i].left {
        if let Some(rt) = self.buf[i].right {
          // both subtrees exist
          if self.balance_factor(lt) <= self.balance_factor(rt) {
            // TODO
            let node = self.push(x, self.buf[i].height,

  }

  // Check if something is contained.
  //
  // O(log(N))
  pub fn contains(&self, x: &T) -> bool where T: Ord + std::fmt::Display {
    eprintln!("searching {}", x);
    self.contains_(0, x)
  }

  // Check if something is contained.
  //
  // O(log(N))
  fn contains_(&self, i: usize, x: &T) -> bool where T: Ord + std::fmt::Display {
    eprintln!("  reading {}", self.buf[i].value);
    if *x == self.buf[i].value {
      true
    } else if *x <= self.buf[i].value {
      if let Some(lc) = self.buf[i].left {
        self.contains_(lc, x)
      } else {
        false
      }
    } else {
      if let Some(rc) = self.buf[i].right {
        self.contains_(rc, x)
      } else {
        false
      }
    }
  }
}

// A heap cell. Basically, a heap cell contains a value and possibly two indexes in the buffer
// to know where to get the next cells.
#[derive(Debug)]
struct HeapNode<T> {
  value: T,
  height: usize,
  left: Option<usize>,
  right: Option<usize>
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test0() {
    let mut heap = Heap::<i32>::new();

    heap.insert(12);
    heap.insert(3);
    heap.insert(-15);
    heap.insert(0);
    heap.insert(8);

    assert!(heap.contains(&12));
    assert!(heap.contains(&3));
    assert!(heap.contains(&(-15)));
    assert!(heap.contains(&0));
    assert!(heap.contains(&8));

    panic!();
  }
}
