struct Queue {
    data: Vec<i32>,
    head : usize,
    tail : usize
}

const MX : i32 = 1000005;

impl Queue {
    fn new() -> Self{
        let mut v = Vec::new();
        for _ in 0 .. MX { v.push(0); }
        Queue {
            data : v,
            head : 0,
            tail : 0
        }
    }

    fn push(&mut self, x : i32) {
        self.data[self.tail] = x;
        self.tail += 1;
    }

    fn pop(&mut self) {
        self.head += 1;
    }

    fn front(&mut self) -> i32 {
        return self.data[self.head];
    }

    fn back(&mut self) -> i32 {
        return self.data[self.tail - 1];
    }
}
