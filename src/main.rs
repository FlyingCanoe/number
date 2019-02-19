fn main() {
    let x = new(1, 2);
    let mut y = new(1, 2);
    println!("{:?}", y);
    y.mutiply(x);
    print!("{:?}",y);
}

#[derive(Debug)]
struct TrueFlot {
    nominateur: isize,
    denominateur: usize

}
    fn new(n: isize, d: usize) -> TrueFlot {
        TrueFlot {
            nominateur: n,
            denominateur: d, 
        }
    }

impl TrueFlot {
    fn mutiply(&mut self, mutiplyor: TrueFlot) {
        self.nominateur = self.nominateur * mutiplyor.nominateur;
    }
}
