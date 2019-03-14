use crate::fration::Fration;
mod fration;

fn main() {
    let a = Fration::new(2, 1);
    let b = Fration::new(3, 1);
    let y = Fration::new(2, 3);
    eqution_de_premier_degre::EqutionPremierDegre::solve_x(a, b, y)
}

//#[allow(non_camel_case_type)]
pub mod eqution_de_premier_degre {
    use super::fration::Fration;

    // ax+b
    pub struct  EqutionPremierDegre {
        a: Fration,
        b: Fration,
    }

    impl EqutionPremierDegre {
        pub fn solve_x(a: Fration, b: Fration, mut y: Fration) {

            //------//
            //etap 1//
            //------//
            //si a n'est pas égale 1 <=> on afiche a
            if Fration::new(1, 1) != a {
                print!("{}", a)
            }

            print!("x");

            if  !b.is_nul() {
                print!("+{}", b)
            }
            println!(" = y");

            //------//
            //etap 2//
            //------//
            //si a n'est pas égale 1 <=> on afiche a
            if !a.is_one() {
                print!("{}", a)
            }

            print!("x");

            if !b.is_nul() {
                print!("+{}", b)
            }

            println!(" = {}", y);

            //------//
            //etap 3//
            //------//
            //si a n'est pas égale 1 <=> on afiche a
            if !a.is_one() {
                print!("{}", a)
            }

            print!("x");

            print!(" = {}", y);

            if  !b.is_nul() {
                println!("-{}", b)
            }

            //------//
            //etap 4//
            //------//

            y = y-b;
            drop(b);

            if !a.is_nul() {
                print!("{}", a)
            }

            print!("x = {}", y)
        }
    }
}
