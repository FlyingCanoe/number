mod fration;
mod eqution;
use fration::fration::Fration;

fn main() {
    let a = Fration::new(1, 2);
    let b = Fration::new(1, 4);
    println!("{:?}", a*b);
}