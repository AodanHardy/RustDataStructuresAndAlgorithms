pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

pub struct Stack<T> {
    pub top: Option<Box<Node<T>>>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { top: None }
    }

    pub fn push(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,
            next: self.top.take(),
        });
        self.top = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.top.take().map(|node| {
            self.top = node.next;
            node.value
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.top.as_ref().map(|node| &node.value)
    }

    pub fn is_empty(&self) -> bool {
        self.top.is_none()
    }

    pub fn print(&self)
    where
        T: std::fmt::Display,
    {
        let mut out = String::new();
        let mut node = &self.top;

        while let Some(n) = node {
            out.push_str(&format!("{} -> ", n.value));
            node = &n.next;
        }

        out.push_str("None");
        println!("{}", out);
    }
}