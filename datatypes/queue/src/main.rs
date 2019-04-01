use std::mem::replace;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Queue<T> {
  // items store
  vec: Vec<Option<T>>,
  // length of the queue
  length: usize,
  // next place to enqueue
  enqueue_at: usize,
  // next place to dequeue
  dequeue_at: usize,
}

impl<T> Queue<T> {
  // O(1)
  fn new() -> Self {
    Queue {
      vec: Vec::new(),
      length: 0,
      enqueue_at: 0,
      dequeue_at: 0,
    }
  }

  // O(1)
  fn enqueue(&mut self, x: T) where T: std::fmt::Debug {
    if self.enqueue_at == self.vec.len() {
      self.vec.push(Some(x));
      self.enqueue_at += 1;
    } else {
      assert!(self.vec[self.enqueue_at].is_none());
      self.vec[self.enqueue_at] = Some(x);

      let next_i = self.enqueue_at + 1;

      match self.vec.get(next_i) {
        Some(e) if e.is_some() => {
          self.enqueue_at = self.vec.len();
        }

        _ => {
          self.enqueue_at = next_i;
        }
      }
    }

    self.length += 1;
  }

  // O(1)
  fn dequeue(&mut self) -> Option<T> {
    if self.length == 0 {
      None
    } else {
      if self.enqueue_at == self.vec.len() {
        self.enqueue_at = self.dequeue_at;
      }

      let x = replace(&mut self.vec[self.dequeue_at], None);
      self.dequeue_at = (self.dequeue_at + 1) % self.vec.len();
      self.length -= 1;

      x
    }
  }

  // O(1)
  fn peek(&self) -> Option<&T> {
    if self.length == 0 {
      None
    } else {
      self.vec.get(self.enqueue_at - 1).and_then(|i| i.as_ref())
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn pop() {
    let mut q: Queue<i32> = Queue::new();
    q.enqueue(3);
    q.enqueue(-1);
    q.enqueue(17);

    assert_eq!(q.dequeue(), Some(3));
    assert_eq!(q.dequeue(), Some(-1));

    q.enqueue(38457);
    q.enqueue(12);

    assert_eq!(q.dequeue(), Some(17));
    assert_eq!(q.dequeue(), Some(38457));
    assert_eq!(q.dequeue(), Some(12));
  }
}

fn main() {
}
