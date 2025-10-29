pub struct Node<T> {
    pub value: T,
    pub left: Option<Box<Node<T>>>,
    pub right: Option<Box<Node<T>>>,
}

pub struct BinaryTree<T> {
    pub root: Option<Box<Node<T>>>,
}

impl<T: Ord> BinaryTree<T> {
    pub fn new() -> Self {
        BinaryTree { root: None }
    }

    pub fn insert(&mut self, value: T) {
        fn go<T: Ord>(node: &mut Option<Box<Node<T>>>, value: T) {
            match node {
                Some(n) => {
                    if value < n.value {
                        go(&mut n.left, value);
                    } else {
                        go(&mut n.right, value);
                    }
                }
                None => {
                    *node = Some(Box::new(Node { value, left: None, right: None }));
                }
            }
        }
        go(&mut self.root, value);
    }

    pub fn contains(&self, value: &T) -> bool {
        fn find<T: Ord>(node: &Option<Box<Node<T>>>, value: &T) -> bool {
            match node {
                Some(n) => {
                    if value == &n.value {
                        true
                    } else if value < &n.value {
                        find(&n.left, value)
                    } else {
                        find(&n.right, value)
                    }
                }
                None => false,
            }
        }
        find(&self.root, value)
    }

    pub fn print(&self)
    where
        T: std::fmt::Display,
    {
        fn inorder<T: std::fmt::Display>(node: &Option<Box<Node<T>>>, out: &mut String) {
            if let Some(n) = node {
                inorder(&n.left, out);
                out.push_str(&format!("{} ", n.value));
                inorder(&n.right, out);
            }
        }

        let mut s = String::new();
        inorder(&self.root, &mut s);
        println!("{}", s.trim());
    }
}