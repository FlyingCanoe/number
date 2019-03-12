use crate::fration::Fration;

mod fration;

fn main() {

}

enum variable {
    x,
    y
}

struct Monombre {
    fration: Fration,
    variable: variable,
    expoxen: i128,

}

#[allow(non_camel_case_type)]
mod eqution_de_premier_degre {
    use super::fration::Fration;
    use super::Monombre;


    struct  EqutionPremierDegre {
        x: Monombre,
        y: Monombre,
        c: Fration,
    }

