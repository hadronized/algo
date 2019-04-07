#[derive(Debug)]
struct Heap<T> {
  buf: Vec<T>
}

impl<T> Heap<T> {
  /// Create an empty heap.
  ///
  /// O(1)
  pub fn new() -> Self {
    Heap {
      buf: Vec::new()
    }
  }

  /// Number of elements in the heap.
  ///
  /// O(1)
  #[inline(always)]
  pub fn len(&self) -> usize {
    self.buf.len()
  }

  // Get the parent index of an indexed node.
  #[inline(always)]
  fn parent_index(i: usize) -> usize {
    (i - 1) >> 1
  }

  // Up head until heap property is restored.
  //
  // O(h).
  fn up_head(&mut self, mut i: usize) where T: Ord {
    while i != 0 {
      let parent_i = Self::parent_index(i);

      if self.buf[parent_i] < self.buf[i] {
        self.buf.swap(parent_i ,i);
      } else {
        break;
      }

      i = parent_i;
    }
  }

  /// Enqueue an element in the heap.
  ///
  /// O(log(n))
  pub fn enqueue(&mut self, x: T) where T: Ord {
    let i = self.len();

    self.buf.push(x);
    self.up_head(i);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test0() {
    let mut heap = Heap::<i32>::new();

    heap.enqueue(12); println!("{:?}", heap);
    heap.enqueue(3); println!("{:?}", heap);
    heap.enqueue(-15); println!("{:?}", heap);
    heap.enqueue(0); println!("{:?}", heap);
    heap.enqueue(8); println!("{:?}", heap);
    heap.enqueue(4); println!("{:?}", heap);

    //assert!(heap.contains(&12));
    //assert!(heap.contains(&3));
    //assert!(heap.contains(&(-15)));
    //assert!(heap.contains(&0));
    //assert!(heap.contains(&8));

    panic!();
  }
}
