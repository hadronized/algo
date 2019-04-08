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
  fn parent(i: usize) -> usize {
    (i - 1) >> 1
  }

  // Get the children indexes of an indexed node.
  #[inline(always)]
  fn children(i: usize) -> (usize, usize) {
    let k = i << 1;
    (k + 1, k + 2)
  }

  // Up head until heap property is restored.
  //
  // O(h).
  fn up_head(&mut self, mut i: usize) where T: Ord {
    while i != 0 {
      let parent_i = Self::parent(i);

      if self.buf[parent_i] < self.buf[i] {
        self.buf.swap(parent_i ,i);
      } else {
        break;
      }

      i = parent_i;
    }
  }

  // Down heap until heap property is restored.
  fn down_head(&mut self) -> Option<T> where T: Ord {
    if self.buf.is_empty() {
      None
    } else {
      let old_root = self.buf.swap_remove(0);
      let mut i = 0;

      while i < self.buf.len() {
        let (left, right) = Self::children(i);

        if left < self.len() && self.buf[i] < self.buf[left] {
          if right < self.len() && self.buf[i] < self.buf[right] {
            // both left and right violated
            if self.buf[left] < self.buf[right] {
              self.buf.swap(i, right);
              i = right;
            } else {
              self.buf.swap(i, left);
              i = left;
            }
          } else {
            // only left violated
            self.buf.swap(i, left);
            i = left;
          }
        } else if right < self.len() && self.buf[i] < self.buf[right] {
          // only right violated
          self.buf.swap(i, right);
          i = right;
        } else {
          // property is okay
          break;
        }
      }

      Some(old_root)
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

  /// Dequeue an element from the heap.
  pub fn dequeue(&mut self) -> Option<T> where T: Ord {
    self.down_head()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test0() {
    let mut heap = Heap::<i32>::new();

    heap.enqueue(12);
    heap.enqueue(3);
    heap.enqueue(-15);
    heap.enqueue(0);
    heap.enqueue(8);
    heap.enqueue(4);

    assert_eq!(heap.dequeue(), Some(12));
    assert_eq!(heap.dequeue(), Some(8));
    assert_eq!(heap.dequeue(), Some(4));
    assert_eq!(heap.dequeue(), Some(3));
    assert_eq!(heap.dequeue(), Some(0));
    assert_eq!(heap.dequeue(), Some(-15));
  }

  #[test]
  fn test1() {
    use std::cmp::Reverse;

    let mut heap = Heap::<Reverse<i32>>::new();

    heap.enqueue(Reverse(12));
    heap.enqueue(Reverse(3));
    heap.enqueue(Reverse(-15));
    heap.enqueue(Reverse(0));
    heap.enqueue(Reverse(8));
    heap.enqueue(Reverse(4));

    assert_eq!(heap.dequeue(), Some(Reverse(-15)));
    assert_eq!(heap.dequeue(), Some(Reverse(0)));
    assert_eq!(heap.dequeue(), Some(Reverse(3)));
    assert_eq!(heap.dequeue(), Some(Reverse(4)));
    assert_eq!(heap.dequeue(), Some(Reverse(8)));
    assert_eq!(heap.dequeue(), Some(Reverse(12)));
  }
}
