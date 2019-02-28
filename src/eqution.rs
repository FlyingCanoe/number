use super::fration::Fration;


#[derive(PartialEq, Debug)]
struct StructVar {
    lettre: char,
    expoxen: i128,
}

#[derive(PartialEq, Debug)]
enum Var {
    Var_presente(StructVar),
    Var_absente,
}

#[derive(Debug)]
struct Monombre {
    qutian: Fration,
    var: Var
}

impl Monombre {
    //determine si un monumbre peut étre assosier
    // a un autre monombre pour étre simplifer
    fn compatible(self, autre_monombre: Monombre) -> bool {
        if self.var == autre_monombre.var {
            return true
        }
        else {return false}
    }
}

pub enum ElementAlgerbryque {
    Enum_Monombre(Monombre),
    OuvertureDeParentaise, 
    FemeturDeParentaise,
    Mutiplucation,
    Divison,
    Adition,
    Soutration
}

pub struct Eqution {
    list_d_element: Vec<ElementAlgerbryque>
}

impl Eqution {
    pub fn new(list_element: Vec<ElementAlgerbryque>) -> Eqution {
        Eqution {
            list_d_element: list_element
        }
    }

    // trie les element algerbryque
    pub fn simplifer_etap1(&self) -> Vec<Vec<Monombre>>   {
        // une liste de groupe de monombre compatible
        let mut liste_groupe_de_monombre :Vec<Vec<Monombre>> = vec![];

        for element in &self.list_d_element {
            for groupe in &mut liste_groupe_de_monombre {

                //pour savoir si un variable et compatible il faut savoir s'ils on la méme variable,
                // vu qu'un group on tous la méme variable, il sufi de prend un representan pour le verifer
                let representant = &groupe[0].var;

                let var_de_element = match element  {
                    ElementAlgerbryque::Enum_Monombre(monombre) => monombre.var.clone(),
                    _ => panic!("bug: les eqution devrait toujoure comencer avec un monombre")
                };

                let monombre_de_element = match element {
                    ElementAlgerbryque::Enum_Monombre(monombroe) => monombroe,
                    _ => panic!("")
                };

                if *representant ==   var_de_element {
                    groupe.push(monombre_de_element.clone())
                }
            }
        }
    liste_groupe_de_monombre
    }
}

//aprés cette linge ce n'ait que du code lier à la sementique de rust
impl Clone for Var {
    fn clone(&self) -> Var {

        // returne un varable absente si lavarable cloner est absent
        if let Var::Var_absente = self {
            return {Var::Var_absente}
        }

        Var::Var_presente(StructVar {
            lettre:  match self {
                Var::Var_presente(var) => var.lettre.clone(),
                _ => panic!(), // cela n'arivra jamait, car si la varable est absente,
                               // il est traiter a la linge 75
            },
            expoxen: match self {
                Var::Var_presente(var) => var.expoxen.clone(),
                _ => panic!(), // cela n'arivra jamait, car si la varable est absente,
                               // il est traiter a la linge 75
            },
        })
    }
}

impl Clone for Monombre {
    fn clone(&self) -> Monombre {
        Monombre {
            qutian: self.qutian.clone(),
            var: self.var.clone(),
        }
    }
}