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

  // Push a value at the end of the buffer and get a node index.
  //
  // O(1)
  fn push(&mut self, x: T, left: Option<usize>, right: Option<usize>) -> usize {
    let i = self.buf.len();
    self.buf.push(HeapNode {
      value: x,
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
      self.push(x, None, None);
    } else {
      self.insert_(0, x);
    }
  }

  // Insert something in the heap.
  //
  // O(log(N))
  fn insert_(&mut self, i: usize, x: T) where T: Ord {
    if x <= self.buf[i].value {
      if let Some(lc) = self.buf[i].left {
        self.insert_(lc, x);
      } else {
        let lc = self.push(x, None, None);
        self.buf[i].left = Some(lc);
      }
    } else {
      if let Some(rc) = self.buf[i].right {
        self.insert_(rc, x);
      } else {
        let rc = self.push(x, None, None);
        self.buf[i].right = Some(rc);
      }
    }
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
