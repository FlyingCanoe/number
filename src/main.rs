use crate::fration::Fration;
extern crate num_rational;
use num_rational::Rational;

mod fration;
mod eqution_de_premier_degre;

fn main() {
    let a = Rational::new(2, 1);
    let b = Rational::new(10, 1);
    let y = Rational::new(2, 1);
    eqution_de_premier_degre::solve_x(a, b, y)
}


pub mod eqution_de_second_degre {
    use num_rational::Rational;
    pub fn solve(a: Rational, b: Rational, c: Rational) {
        println!("{}x^2+{}x+{} = y", a, b, c);
        println!("mn  = ac");
        println!("m+n = b");
        let mn = a*c;
        println!("mn  = {}", mn);
        println!("m+n = {}", b);

        let v :Vec<(isize, isize)> = vec![];
        for i in 1..b as isize {
            println!("{} {}", i, 10-i)
        }
    }
}
