


struct Queue<T> {
    items: Vec<T>
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            items: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.items.len() == 0
    }

    pub fn enqueue(&mut self, new_item: T) {
        self.items.push(new_item)
    }

    pub fn peek(&self) -> Option<&T> {
        self.items.first()
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            Some(self.items.remove(0))
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_queue_is_empty() {
        let queue: Queue<i32> = Queue::new();
        assert!(queue.is_empty());
    }

    #[test]
    fn enqueue_adds_element_to_end() {
        let mut queue = Queue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        assert_eq!(queue.peek(), Some(&1));
    }

    #[test]
    fn dequeue_removes_element_from_front() {
        let mut queue = Queue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.peek(), Some(&2));
    }

    #[test]
    fn dequeue_on_empty_queue_returns_none() {
        let mut queue: Queue<i32> = Queue::new();
        assert_eq!(queue.dequeue(), None);
    }

    #[test]
    fn peek_does_not_remove_element_from_front() {
        let mut queue = Queue::new();
        queue.enqueue(1);
        assert_eq!(queue.peek(), Some(&1));
        assert_eq!(queue.dequeue(), Some(1));
    }
}

