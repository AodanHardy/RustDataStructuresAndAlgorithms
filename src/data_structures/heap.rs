pub struct Heap<T> {
    data: Vec<T>,
}

impl<T: Ord> Heap<T> {
    pub fn new() -> Self {
        Heap { data: Vec::new() }
    }

    pub fn insert(&mut self, value: T) {
        self.data.push(value);
        let mut i = self.data.len() - 1;
        while i > 0 {
            let parent = (i - 1) / 2;
            if self.data[i] < self.data[parent] {
                self.data.swap(i, parent);
                i = parent;
            } else {
                break;
            }
        }
    }

    pub fn remove_min(&mut self) -> Option<T> {
        if self.data.is_empty() {
            return None;
        }

        let last = self.data.pop().unwrap();
        if self.data.is_empty() {
            return Some(last);
        }

        let min = std::mem::replace(&mut self.data[0], last);
        self.heapify(0);
        Some(min)
    }

    fn heapify(&mut self, i: usize) {
        let left = 2 * i + 1;
        let right = 2 * i + 2;
        let mut smallest = i;

        if left < self.data.len() && self.data[left] < self.data[smallest] {
            smallest = left;
        }
        if right < self.data.len() && self.data[right] < self.data[smallest] {
            smallest = right;
        }

        if smallest != i {
            self.data.swap(i, smallest);
            self.heapify(smallest);
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.data.first()
    }

    pub fn print(&self)
    where
        T: std::fmt::Display,
    {
        for x in &self.data {
            print!("{} ", x);
        }
        println!();
    }
}