use std::ops::Mul;
use std::ops::Div;
use std::ops::Add;
use std::ops::Sub;

fn main() {
    let x = new(1, 2);
    let mut y = new(1, 2);
    println!("{:?}", y);
    y = x-y;
    print!("{:?}", y);
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

impl Mul<TrueFlot> for TrueFlot {
    type Output = TrueFlot;

    fn mul(self, other: TrueFlot) -> TrueFlot {
        TrueFlot {
            nominateur: self.nominateur * other.nominateur,
            denominateur: self.denominateur * other.denominateur,
        } 
    }
}

impl Div<TrueFlot> for TrueFlot {
    type Output = TrueFlot;

    fn div(self, other: TrueFlot) -> TrueFlot  {
        TrueFlot {
            nominateur: self.nominateur * other.denominateur as isize,
            denominateur: self.denominateur * other.denominateur,
        }
    }
}

impl Add<TrueFlot> for TrueFlot {
    type Output = TrueFlot;

    fn add(self, other: TrueFlot) -> TrueFlot {
        TrueFlot {
            nominateur: (self.nominateur * other.denominateur as isize) + 
                        (other.nominateur * self.denominateur as isize),

            denominateur: self.denominateur * other.denominateur,
        } 
    }
}

impl Sub<TrueFlot> for TrueFlot {
    type Output = TrueFlot;

    fn sub(self, other: TrueFlot) -> TrueFlot {
        TrueFlot {
            nominateur: (self.nominateur * other.denominateur as isize) - 
            (other.nominateur * self.denominateur as isize),

            denominateur: self.denominateur * other.denominateur,
        }
    }

}

impl TrueFlot {
    fn diviseur(number: isize) -> Vec<usize> {
        let vec: Vec<usize> = vec!();

        for i in 2..number {
            println!("{}", number/i);
        }
        vec
    }
}