pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

pub struct LinkedList<T> {
    pub head: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    pub fn append(&mut self, value: T) {
        let new_node = Box::new(Node { value, next: None });

        match self.head.as_mut() {
            Some(mut current) => {
                while current.next.is_some() {
                    current = current.next.as_mut().unwrap();
                }
                current.next = Some(new_node);
            }
            None => {
                self.head = Some(new_node);
            }
        }
    }

    pub fn insert(&mut self, index: usize, value: T) {
        if index == 0 {
            let new_node = Box::new(Node {
                value,
                next: self.head.take(),
            });
            self.head = Some(new_node);
            return;
        }

        let mut current = &mut self.head;
        let mut pos = 0;

        while let Some(node) = current {
            if pos + 1 == index {
                let next = node.next.take();
                node.next = Some(Box::new(Node { value, next }));
                return;
            }
            pos += 1;
            current = &mut node.next;
        }

        // if index is out of bounds, just append
        self.append(value);
    }

    pub fn print(&self)
    where
        T: std::fmt::Display,
    {
        let mut out = String::new();
        let mut node = &self.head;

        while let Some(n) = node {
            out.push_str(&format!("{} -> ", n.value));
            node = &n.next;
        }

        out.push_str("None");
        println!("{}", out);
    }
}