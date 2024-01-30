

struct Stack<T> {
    capacity: i32,
    items: Vec<T>
}

impl<T> Stack<T> {

    pub fn new() -> Stack<T> {
        Stack {
            capacity: 10,
            items: vec![],
        }
    }

    pub fn is_empty(&self) -> bool {
        self.items.len() == 0
    }

    pub fn push(&mut self, item: T) {
        if self.capacity > (self.items.len() + 1) as i32 {
            self.items.push(item)
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.items.last()
    }

    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_stack_is_empty() {
        let stack: Stack<i32> = Stack::new();
        assert!(stack.is_empty());
    }

    #[test]
    fn push_adds_element_to_top() {
        let mut stack = Stack::new();
        stack.push(1);
        assert_eq!(stack.peek(), Some(&1));
    }

    #[test]
    fn pop_removes_top_element() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.peek(), Some(&1));
    }

    #[test]
    fn pop_on_empty_stack_returns_none() {
        let mut stack: Stack<i32> = Stack::new();
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn peek_does_not_remove_top_element() {
        let mut stack = Stack::new();
        stack.push(1);
        assert_eq!(stack.peek(), Some(&1));
        assert_eq!(stack.pop(), Some(1));
    }
}
