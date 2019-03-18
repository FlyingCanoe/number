use crate::fration::Fration;
mod fration;
mod eqution_de_premier_degre;

fn main() {
    let a = Fration::new(2, 1);
    let b = Fration::new(10, 1);
    let c = Fration::new(2, 1);

    eqution_de_second_degre::solve(a, b, c);
}


pub mod eqution_de_second_degre {
    use crate::fration::Fration;
    pub fn solve(a: Fration, b: Fration, c: Fration) {
        println!("{}x^2+{}x+{} = y", a, b, c);
        println!("mn  = ac");
        println!("m+n = b");
        let mn = a*c;
        println!("mn  = {}", mn);
        println!("m+n = {}", b);

        let v :Vec<(i64, i64)> = vec![];
        for i in 1..b.nominateur/b.denominateur {
            println!("{} {}", i, 10-i)
        }
    }
}
