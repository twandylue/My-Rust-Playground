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
            true => None,
            false => Some(self.items.remove(0)),
        }
    }

    pub fn size(&self) -> usize {
        self.items.len()
    }

    pub fn peek(&self) -> Option<&T> {
        self.items.first()
    }
}

pub struct Stack<T> {
    maxsize: usize,
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new(maxsize: usize) -> Self {
        Self {
            maxsize,
            items: Vec::with_capacity(maxsize),
        }
    }

    pub fn push(&mut self, item: T) -> bool {
        if self.items.len() == self.maxsize {
            return false;
        }
        self.items.push(item);
        return true;
    }

    pub fn pop(&mut self) -> Option<T> {
        return self.items.pop();
    }

    pub fn size(&self) -> usize {
        return self.items.len();
    }

    pub fn peek(&self) -> Option<&T> {
        return self.items.last();
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

    #[test]
    fn stack_ok() {
        let mut st: Stack<u8> = Stack::new(10);
        st.push(1);
        st.push(2);
        st.push(3);
        st.push(4);
        st.push(5);
        assert_eq!(st.maxsize, 10);
        assert_eq!(st.pop().unwrap(), 5);
        assert_eq!(st.push(6), true);
    }
}
