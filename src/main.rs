use crate::fration::Fration;
mod fration;
mod eqution_de_premier_degre;

fn main() {
    let a = Fration::new(2, 1);
    let b = Fration::new(3, 1);
    let y = Fration::new(2, 3);
    eqution_de_premier_degre::EqutionPremierDegre::solve_x(a, b, y)
}


pub mod eqution_de_second_degre {

}
