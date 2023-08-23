use std::collections::LinkedList;

pub struct Queue<T>(LinkedList<T>);

impl<T> Queue<T> {
  pub fn new() -> Self {
    Queue(LinkedList::new())
  }
  pub fn enqueue(mut self, item: T) {
    self.0.push_back(item)
  }
  pub fn dequeue(mut self) -> Option<T> {
    self.0.pop_front()
  }
  pub fn is_empty(self) -> bool {
    self.0.is_empty()
  }
  pub fn len(self) -> usize {
    self.0.len()
  }
}
