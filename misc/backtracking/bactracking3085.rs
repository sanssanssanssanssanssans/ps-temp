struct Backtracking {
    ret : i32,
    bombo : Vec<Vec<char>>,
    n : usize
}

impl Backtracking {
    fn new(n : usize, bombo : Vec<Vec<char>>) -> Self {
        Backtracking {
             n : n,
             bombo : bombo,
            ret : 0
        }
    }

    fn solve(&mut self) {
        for i in 0 .. self.n{
            let mut cnt = 1;
            for j in 0 .. self.n-1 {
                if self.bombo[i][j] == self.bombo[i][j+1] {
                    cnt += 1;
                    self.ret = self.ret.max(cnt);
                } else {
                    cnt = 1;
                }
            }
        }
        for i in 0 .. self.n {
            let mut cnt = 1;
            for j in 0 .. self.n - 1 {
                if self.bombo[j][i] == self.bombo[j+1][i] {
                    cnt += 1;
                    self.ret = self.ret.max(cnt);
                } else {
                    cnt = 1;
                }
            }
        }
    }
}
