fn main() {
    let mut q: Queue<String> = Queue::new();
    q.enqueue(String::from("first"));
    q.enqueue(String::from("second"));
    q.dequeue();
    println!("length: {}", q.size())
}

// basic implement: enqueue, dequeue, peek and size
pub struct Queue<T> {
    items: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn enqueue(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        match self.items.is_empty() {
            false => Some(self.items.remove(0)),
            true => None,
        }
    }

    pub fn size(&self) -> usize {
        self.items.len()
    }

    pub fn peek(&self) -> Option<&T> {
        self.items.first()
    }
}
