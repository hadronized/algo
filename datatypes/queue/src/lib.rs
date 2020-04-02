const BLOCK_SIZE: usize = 5;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Queue<T> {
  buf: Vec<Option<T>>,
  enqueue_at: usize,
  dequeue_at: usize,
}

impl<T> Queue<T> {
  pub fn new() -> Self {
    Queue {
      buf: Vec::new(),
      enqueue_at: 0,
      dequeue_at: 0,
    }
  }

  fn grow(&mut self) {
    assert_eq!(self.enqueue_at, self.dequeue_at);

    // the idea of the algorithm is pretty simple: we look at where the index is located in
    // the buffer and we move only the smaller part -> that means less copies so better performance
    //
    // by definition, we use self.enqueue_at as index since it equals self.dequeue_at
    //
    // we resize the buffer first
    let prev_len = self.buf.len();
    self
      .buf
      .resize_with(prev_len + BLOCK_SIZE, Default::default);
    let new_len = self.buf.len();

    if self.enqueue_at <= new_len {
      // the left part is smaller, which means that we need to move them just after the right part
      // and update the enqueue index
      for i in 0..self.enqueue_at {
        self.buf.swap(i, prev_len + i);
      }

      self.enqueue_at = self.dequeue_at + prev_len;
    } else {
      // the right part is smaller, which means that we need to move them at the very end of the
      // new buffer and update the dequeue_at index
      for i in self.enqueue_at..prev_len {
        self.buf.swap(i, new_len - prev_len + i);
      }

      self.dequeue_at = new_len - prev_len + self.enqueue_at;
    }
  }

  pub fn len(&self) -> usize {
    let l = (self.enqueue_at as isize - self.dequeue_at as isize).abs() as usize;

    if l == 0 {
      // if l == 0, it means that we’re either completely full or completely empty; that property
      // can be checked by looking up buf[0]: if it’s None, then it’s completely empty, otherwise
      // it’s completely full
      self
        .buf
        .first()
        .map(|v| if v.is_some() { self.buf.len() } else { 0 })
        .unwrap_or(0)
    } else {
      l
    }
  }

  pub fn push(&mut self, x: T) {
    // no more available place; we need to grow first
    if self.buf.is_empty()
      || (self.enqueue_at == self.dequeue_at && self.buf[self.enqueue_at].is_some())
    {
      self.grow();
    }

    self.buf[self.enqueue_at] = Some(x);
    self.enqueue_at = (self.enqueue_at + 1) % self.buf.len();
  }

  pub fn pop(&mut self) -> Option<T> {
    if self.len() == 0 {
      None
    } else {
      let r = self.buf[self.dequeue_at].take();
      self.dequeue_at = (self.dequeue_at + 1) % self.buf.len();
      r
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    let mut q = Queue::new();
    assert_eq!(q.len(), 0);

    eprintln!("-> {:?}", q);
    q.push(0);
    assert_eq!(q.len(), 1);

    eprintln!("-> {:?}", q);
    q.push(1);
    assert_eq!(q.len(), 2);

    eprintln!("-> {:?}", q);
    q.push(2);
    assert_eq!(q.len(), 3);

    eprintln!("-> {:?}", q);
    q.push(3);
    assert_eq!(q.len(), 4);

    eprintln!("-> {:?}", q);
    assert_eq!(q.pop(), Some(0));
    assert_eq!(q.len(), 3);

    eprintln!("-> {:?}", q);
    assert_eq!(q.pop(), Some(1));
    assert_eq!(q.len(), 2);

    eprintln!("-> {:?}", q);
    q.push(4);
    q.push(5);
    q.push(6);
    assert_eq!(q.len(), 5);

    eprintln!("-> {:?}", q);
    q.push(7);
    assert_eq!(q.len(), 6);

    eprintln!("-> {:?}", q);

    assert_eq!(q.pop(), Some(2));
    assert_eq!(q.pop(), Some(3));
    assert_eq!(q.pop(), Some(4));
    assert_eq!(q.pop(), Some(5));
    assert_eq!(q.pop(), Some(6));
    eprintln!("-> {:?}", q);
    assert_eq!(q.len(), 1);
    assert_eq!(q.pop(), Some(7));
    eprintln!("-> {:?}", q);
    assert_eq!(q.len(), 0);

    eprintln!("-> {:?}", q);
  }
}
