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
    nominateur: i64,
    denominateur: i64,

}
    fn new(n: i64, d: i64) -> TrueFlot {
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
            nominateur: self.nominateur * other.denominateur as i64,
            denominateur: self.denominateur * other.denominateur,
        }
    }
}

impl Add<TrueFlot> for TrueFlot {
    type Output = TrueFlot;

    fn add(self, other: TrueFlot) -> TrueFlot {
        TrueFlot {
            nominateur: (self.nominateur * other.denominateur as i64) + 
                        (other.nominateur * self.denominateur as i64),

            denominateur: self.denominateur * other.denominateur,
        } 
    }
}

impl Sub<TrueFlot> for TrueFlot {
    type Output = TrueFlot;

    fn sub(self, other: TrueFlot) -> TrueFlot {
        TrueFlot {
            nominateur: (self.nominateur * other.denominateur as i64) - 
                        (other.nominateur * self.denominateur as i64),

            denominateur: self.denominateur * other.denominateur,
        }
    }

}

impl TrueFlot {
    fn diviseur(number: i64) -> Vec<i64> {
        let mut vec: Vec<i64> = vec!();

        for i in 2..number {
            let a = number as f64/i as f64;

            if a.trunc() == a {
                vec.push(a as i64)
            }
        }
    vec
    }
    fn simplifer(self: mut &self) {
        let diviseur_nominateur = diviseur(self.nominateur);
        let diviseur_denominateur = diviseur(self.denominateur);

        for diviseur_nominateur.rev() {
            
        }
    }
}
