use super::fration::Fration;
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