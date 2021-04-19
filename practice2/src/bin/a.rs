use proconio::{input, fastout};

#[fastout]
fn main() {
    input!{
        n: usize,
        q: usize,
    }
    let mut uf = UnionField::new(n);
    for _ in 0..q{
        input!{
            t: usize, u: usize, v: usize,
        }
        match t{
            0 => uf.merge(u, v),
            _ => println!("{}" ,if uf.is_same(u, v) {1} else {0}),
        }
    }
}


pub struct UnionField {
    n: usize,
    root: Vec<usize>,
    size: Vec<usize>,
}

impl UnionField{
    pub fn new(s: usize) -> UnionField{
        UnionField{
        n: s,
        root: (0..s).map(|i| i).collect::<Vec<usize>>(),
        size: vec![1; s],
        }
    }

    pub fn find(&mut self, x: usize) -> usize{
        if x == self.root[x] {
            return x;
        }else{
            self.root[x] = self.find(self.root[x]);
            return self.root[x];
        }
    }
    
    pub fn merge(&mut self, x: usize, y: usize){
        let rx = self.find(x);
        let ry = self.find(y);
        if rx==ry{return;}
        let (rb, rs) = if self.size[rx]<self.size[ry] {(rx, ry)} else {(ry, rx)};
        self.root[rb] = rs;
        self.size[rb] += self.size[rs];
        self.size[rs] = 0;
        //self.n -= 1;
    }

    pub fn is_same(&mut self, x: usize, y: usize) -> bool{
        if self.find(x)==self.find(y){return true;}
        else {return false;}
    }
}