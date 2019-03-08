use crate::eqution::{ElementAlgerbryque, Monombre, Eqution};
use crate::fration::Fration;

mod eqution;
mod fration;


fn main() {
    let a :Vec<ElementAlgerbryque> = vec![
        ElementAlgerbryque::EnumMonombre(Monombre::new(fration::new(1, 1), Some('x'), 1)),
        ElementAlgerbryque::Adition,
        ElementAlgerbryque::EnumMonombre(Monombre::new(fration::new(2, 1), Some('x'), 1)),
        ElementAlgerbryque::Adition,
        ElementAlgerbryque::EnumMonombre(Monombre::new(fration::new(1, 1), Some('y'), 1)),
        ElementAlgerbryque::Adition,
        ElementAlgerbryque::EnumMonombre(Monombre::new(fration::new(3, 1), Some('x'), 1)),
        ElementAlgerbryque::Adition,
        ElementAlgerbryque::EnumMonombre(Monombre::new(fration::new(2, 1), None, 2)),
        ElementAlgerbryque::Egaliter,
        ElementAlgerbryque::EnumMonombre(Monombre::new(fration::new( 0, 1), None, 1)),
    ];
    let mut b = Eqution::new(a);
    println!("{}", b);
    b.simplifer();
    println!("{}", b);
}