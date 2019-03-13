use crate::fration::Fration;
mod fration;

fn main() {
    let a = Fration::new(1, 1);
    let b = Fration::new(0, 1);
    eqution_de_premier_degre::EqutionPremierDegre::solve(a, b)
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
        pub fn solve(a: Fration, b: Fration) {

            //si a n'est pas égale 1 <=> on afiche a
            if Fration::new(1, 1) != a {
                print!("{}", a)
            }

            print!("x");

            if Fration::new(0, 1) != b {
                print!("+{}", b)
            }
            println!(" = 0");

        }
    }
}
