// NOTE: basic implement: enqueue, dequeue, peek and size

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enqueue_ok() {
        let mut qu: Queue<u8> = Queue::new();
        qu.enqueue(1);
        qu.enqueue(8);
        qu.enqueue(200);
        assert_eq!(qu.size(), 3);

        let p = qu.peek().unwrap();
        assert_eq!(*p, 1);
        let f: u8 = qu.dequeue().unwrap();
        assert_eq!(f, 1);
    }
}
