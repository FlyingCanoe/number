use crate::eqution::{ElementAlgebrique, Monombre, Equation};
use crate::eqution::simplification;
mod eqution;
mod fration;


fn main() {
    let a :Vec<ElementAlgebrique> = vec![
        ElementAlgebrique::EnumMonombre(Monombre::new(fration::new(1, 1), Some('x'), 1)),
        ElementAlgebrique::Adition,
        ElementAlgebrique::EnumMonombre(Monombre::new(fration::new(2, 1), Some('x'), 1)),
        ElementAlgebrique::Adition,
        ElementAlgebrique::EnumMonombre(Monombre::new(fration::new(1, 1), Some('y'), 1)),
        ElementAlgebrique::Adition,
        ElementAlgebrique::EnumMonombre(Monombre::new(fration::new(3, 1), Some('x'), 1)),
        ElementAlgebrique::Soutration,
        ElementAlgebrique::EnumMonombre(Monombre::new(fration::new(2, 1), None, 2)),
        ElementAlgebrique::Egaliter,
        ElementAlgebrique::EnumMonombre(Monombre::new(fration::new(0, 1), None, 1)),
    ];
    let mut b = Equation::new(a);
    println!("{}", b);
    simplification::run(&mut b);
    println!("{}", b);
}