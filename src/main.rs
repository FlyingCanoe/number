mod eqution;
mod fration;


fn main() {
    let a = fration::Fration::new(1, 2);
    let b = fration::Fration::new(1, 4);
    println!("{:?}", a*b);
}