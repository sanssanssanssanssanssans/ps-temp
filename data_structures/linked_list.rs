struct LinkedList {
    data : Vec<usize>,
    pre : Vec<usize>,
    nxt : Vec<usize>,
    unused : usize
}

impl LinkedList {
    fn new(mx : usize) -> Self {
        let size = mx as usize;
        LinkedList {
            data: vec![0; size],
            pre: vec![987654321; size],
            nxt: vec![987654321; size],
            unused: 0,
        }
    }

    fn insert(&mut self, addr : usize, num : usize) {
        self.data[self.unused] = num;
        self.pre[self.unused] = addr;
        self.nxt[self.unused] = self.nxt[addr];
        if self.nxt[addr] != 987654321 {
            self.pre[self.nxt[addr]] = self.unused;
        }
        self.nxt[addr] = self.unused;
        self.unused += 1;
    }

    fn erase(&mut self, addr : usize)  {
        self.nxt[self.pre[addr]] = self.nxt[addr];
        if self.nxt[addr] != 987654321 {
            self.pre[self.nxt[addr]] = self.pre[addr];
        }
    }
}
