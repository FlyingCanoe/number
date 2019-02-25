mod fration;
use fration::fration::Fration;

fn main() {
    let a = Fration::new(1, 2);
    let b = Fration::new(1, 4);
    println!("{:?}", a*b);
}

struct Var {
    var: char,
    expoxen: i128,

}

impl Clone for Var {
    fn clone(&self) -> Var {
        Var {
            var: self.var.clone(),
            expoxen: self.expoxen.clone()
        }
    }
}

struct Monombre {
    qutian: Option<Fration>,
    var: Option<Var>
}

impl Clone for Monombre {
    fn clone(&self) -> Monombre {
        Monombre {
            qutian: self.qutian.clone(),
            var: self.var.clone(),
        }
    }
}

enum ElementAlgerbryque {
    Monombre(Monombre),
    OuvertureDeParentaise, 
    FemeturDeParentaise,
    Mutiplucation,
    Divison,
    Adition,
    Soutration
}

struct Eqution {
    list_d_element: Vec<ElementAlgerbryque>
}

/*
impl Eqution {
    fn check_pour_fonction_de_droit(self) {
        let x = match self.list_d_element[0].Clone() {
            ElementAlgerbryque::Monombre(monombre) => {
                match monombre {
                    Monombre {qutian, var: Some(i)} => true,
                    _ => false
                }
            },
            _ => false            
        }; 

    }
}*/