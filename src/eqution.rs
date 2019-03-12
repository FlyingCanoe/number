use super::fration::Fration;

pub mod simplification;

#[derive(Debug)]
pub struct Monombre {
    qutian: Fration,
    var_lettre: Option<char>,
    var_expoxen: u32
}

impl Monombre {
    pub fn new(qutian: Fration, var_lettre: Option<char>, exposent: u32) -> Monombre {
        Monombre {
            qutian: qutian,
            var_lettre: var_lettre,
            var_expoxen: exposent,

        }
    }

    //determine si un monumbre peut étre assosier
    // a un autre monombre pour étre simplifer
    fn compatible(&self, autre_monombre: &Monombre) -> bool {
        if self.var_lettre == autre_monombre.var_lettre {

            if self.var_expoxen == autre_monombre.var_expoxen {
                return true
            }
        }
        return false
    }
}

#[derive(Debug)]
pub enum ElementAlgebrique {
    EnumMonombre(Monombre),
    OuvertureDeParentaise, 
    FemeturDeParentaise,
    Egaliter,
    Mutiplucation,
    Divison,
    Adition,
    Soutration
}

#[derive(Debug)]
pub struct Equation {
    list_d_element: Vec<ElementAlgebrique>
}

impl Equation {
    pub fn new(list_element: Vec<ElementAlgebrique>) -> Equation {
        Equation {
            list_d_element: list_element
        }
    }
}

//aprés cette linge ce n'ait que du code lier à la sementique de rust


impl Clone for Monombre {
    fn clone(&self) -> Monombre {
        Monombre {
            qutian: self.qutian.clone(),
            var_lettre: self.var_lettre.clone(),
            var_expoxen: self.var_expoxen
        }
    }
}

impl std::fmt::Display for Monombre {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.var_lettre.is_some() {
            if self.var_expoxen == 1 {
                write!(f, "{}{}", self.qutian, self.var_lettre.unwrap())
            }
            else {
                write!(f, "{}{}{}", self.qutian, self.var_lettre.unwrap(), self.var_expoxen)
            }
        }
        else {
                write!(f, "{}", self.qutian,)
        }
    }
}

impl  std::fmt::Display for ElementAlgebrique {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ElementAlgebrique::EnumMonombre(monombre) => write!(f, "{}", monombre),
            ElementAlgebrique::Adition => write!(f, "+"),
            ElementAlgebrique::Soutration => write!(f, "-"),
            ElementAlgebrique::Divison => write!(f, "/"),
            ElementAlgebrique::Mutiplucation => write!(f, "* "),
            ElementAlgebrique::OuvertureDeParentaise => write!(f, "("),
            ElementAlgebrique::FemeturDeParentaise => write!(f, ")"),
            ElementAlgebrique::Egaliter => write!(f, " = ")
        }

    }
}

impl  std::fmt::Display for Equation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for element_algerbryque in &self.list_d_element {
            print!("{}", element_algerbryque);
        }
        write!(f, "")
    }
}

impl ElementAlgebrique {
    fn unwrap(&self) -> Monombre {
        match self {
            ElementAlgebrique::EnumMonombre(monombre) => monombre.clone(),
            _ => panic!(),

        }
    }
}