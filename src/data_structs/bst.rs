
struct BST<T> where T: PartialOrd {
    root: Option<Box<Node<T>>>,
}

impl<T> BST<T> where T: PartialOrd {
    pub fn new() -> BST<T> {
        BST { root: None }
    }

    pub fn insert(&mut self, key: T) {
        // By default, match statements consume all they can, so when you want to match to only be a reference, you
        // can use the ref keyword so match doesn't take ownership
        match self.root {
            Some(ref mut root) => root.insert(key),
            None => self.root = Some(Box::new(Node::new(key))),
        }
    }

    pub fn contains(&self, key: &T) -> bool {
        match &self.root {
            Some(node) => node.contains(key),
            None => false,
        }
    }

    pub fn min(&self) -> Option<&T> {
        match &self.root {
            Some(node) => node.min(),
            None => None,
        }
    }

    pub fn max(&self) -> Option<&T> {
        match &self.root {
            Some(node) => node.max(),
            None => None,
        }
    }
}

struct Node<T> where T: PartialOrd {
    key: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> Node<T> where T: PartialOrd {
    pub fn new(key: T) -> Node<T> {
        Node {
            key,
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, key: T) {
        if key < self.key {
            match self.left {
                Some(ref mut left) => left.insert(key),
                None => self.left = Some(Box::new(Node::new(key))),
            }
        } else {
            match self.right {
                Some(ref mut right) => right.insert(key),
                None => self.right = Some(Box::new(Node::new(key))),
            }
        }
    }

    pub fn contains(&self, key: &T) -> bool {
        if key < &self.key {
            match &self.left {
                Some(left) => left.contains(key),
                None => false,
            }
        } else if key > &self.key {
            match &self.right {
                Some(right) => right.contains(key),
                None => false,
            }
        } else {
            true
        }
    }

    pub fn min(&self) -> Option<&T> {
        match &self.left {
            Some(left) => left.min(),
            None => Some(&self.key),
        }
    }

    pub fn max(&self) -> Option<&T> {
        match &self.right {
            Some(right) => right.max(),
            None => Some(&self.key),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_and_contains_works() {
        let mut tree = BST::new();
        tree.insert(5);
        tree.insert(3);
        tree.insert(7);

        assert!(tree.contains(&5));
        assert!(tree.contains(&3));
        assert!(tree.contains(&7));
        assert!(!tree.contains(&8));
    }

    #[test]
    fn min_and_max_works() {
        let mut tree = BST::new();
        tree.insert(5);
        tree.insert(3);
        tree.insert(7);
        tree.insert(6);

        assert_eq!(tree.min(), Some(3).as_ref());
        assert_eq!(tree.max(), Some(7).as_ref());
    }
}
