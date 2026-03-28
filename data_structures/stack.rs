struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack {
            data: Vec::new(),
        }
    }
    fn push(&mut self, value: T) {
        self.data.push(value);
    }
    fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }
    fn top(&self) -> Option<&T> {
        self.data.last()
    }
    fn size(&self) -> usize {
        self.data.len()
    }
    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    fn clear(&mut self) {
        self.data.clear();
    }
}
