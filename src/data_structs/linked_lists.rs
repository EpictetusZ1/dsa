

// NOTE:
// This is confusing, and Rust might not be the best language to implement this in

pub struct LinkedList<'a, T> {
    pub value: Option<T>,
    pub next: Option<&'a Box<LinkedList<'a, T>>>,
}

impl<'a> LinkedList<'a, i32> {
    pub fn new() -> LinkedList<'a, i32>{
        LinkedList {
            value: None,
            next: None,
        }
    }

    pub fn push_left(&mut self, node: &'a mut Box<LinkedList<'a, i32>>) {
        if self.next.is_none() {
            self.next = Some(node);
        } else {
            node.next = self.next;
            self.next = Some(node);
        }
    }

    pub fn create_box(&self, x: i32) -> Box<LinkedList<'a, i32>> {
        Box::new(
            LinkedList {
                value: Some(x),
                next: None
            }
        )
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linked_list_push_left() {
        let mut list: LinkedList<i32> = LinkedList::<i32>::new();
        let mut box_ = list.create_box(1);
        let mut box_2 = list.create_box(2);
        let mut box_3 = list.create_box(3);
        let mut box_4 = list.create_box(4);

        list.push_left(&mut box_);
        list.push_left(&mut box_2);
        list.push_left(&mut box_3);
        list.push_left(&mut box_4);
    }
}