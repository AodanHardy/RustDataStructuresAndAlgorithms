pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

pub struct Queue<T> {
    pub front: Option<Box<Node<T>>>,
    pub back: *mut Node<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            front: None,
            back: std::ptr::null_mut(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        let mut new_node = Box::new(Node { value, next: None });
        let raw_tail: *mut _ = &mut *new_node;

        if self.back.is_null() {
            self.front = Some(new_node);
        } else {
            unsafe {
                (*self.back).next = Some(new_node);
            }
        }

        self.back = raw_tail;
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.front.take().map(|mut node| {
            self.front = node.next.take();

            if self.front.is_none() {
                self.back = std::ptr::null_mut();
            }

            node.value
        })
    }

    pub fn is_empty(&self) -> bool {
        self.front.is_none()
    }

    pub fn print(&self)
    where
        T: std::fmt::Display,
    {
        let mut out = String::new();
        let mut node = &self.front;

        while let Some(n) = node {
            out.push_str(&format!("{} -> ", n.value));
            node = &n.next;
        }

        out.push_str("None");
        println!("{}", out);
    }
}