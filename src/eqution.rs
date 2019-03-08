use super::fration;
use super::fration::Fration;
use std::os::raw::c_char;


#[derive(Debug)]
pub struct Monombre {
    qutian: Fration,
    var_lettre: Option<char>,
    var_expoxen: i128
}

impl Monombre {

    pub fn new(qutian: Fration, var_lettre: Option<char>, exposent: i128) -> Monombre {
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
            else { return false }
        }
        else {return false}
    }
}

#[derive(Debug)]
pub enum ElementAlgerbryque {
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
pub struct Eqution {
    list_d_element: Vec<ElementAlgerbryque>
}

impl Eqution {
    pub fn new(list_element: Vec<ElementAlgerbryque>) -> Eqution {
        Eqution {
            list_d_element: list_element
        }
    }

    pub fn simplifer(&mut self) {
        let output_etap1 = self.simplifer_etap2();
        println!();
        for i in &output_etap1 {
            for x in i {
                print!("{}", x)
            }
            println!()
        }

        let output_etap2 = self.simplifer_etap3(output_etap1);
        let mut output_final = vec![];

        for Monombre in output_etap2 {
            output_final.push(ElementAlgerbryque::EnumMonombre(Monombre))
        }
        *self = Eqution::new(output_final);
    }

    //reduie les monombre qui non pas de variable et qui on un exposent superieur a 1
    pub fn simplifer_etap1(&mut self) {
        for ElementAlgerbryque in self {
            
        }

    }

    // rasenble les monombre en group que l'on peut redure en aditionant
    pub fn simplifer_etap2(&self) -> Vec<Vec<Monombre>> {
        // une liste de groupe de monombre compatible
        let mut liste_groupe_de_monombre: Vec<Vec<Monombre>> = vec![];

        for element in &self.list_d_element {
            let mut group_deja_existen = false;

            //extrail les donner de l'élement pour les utiliser directement
            let monombre_de_element = match element {
                ElementAlgerbryque::EnumMonombre(Monombre) => Monombre,
                ElementAlgerbryque::Adition => continue,
                ElementAlgerbryque::Egaliter => return liste_groupe_de_monombre,
               _ => panic!(),
            };

            for groupe in &mut liste_groupe_de_monombre {

                //pour savoir si un variable et compatible il faut savoir s'ils on la méme variable,
                // vu qu'un group on tous la méme variable, il sufi de prend un representan pour le verifer
                let representant = &groupe[0];


                if representant.compatible(&monombre_de_element)  {
                        groupe.push(monombre_de_element.clone());
                        group_deja_existen = true;

                        break // le group a été trouver et un monombre
                              // ne peut étre que dans un group, donc break
                }
            }
            if group_deja_existen == false {

                //le group n'existen pas on le crée
                liste_groupe_de_monombre.push(vec![element.unwrap()]);
                }
            }
        liste_groupe_de_monombre
    }

   pub fn simplifer_etap3(&self, input_vec: Vec<Vec<Monombre>>) ->  Vec<Monombre> {
       let mut output_vec :Vec<Monombre> = vec![];

       //simplifi chaque groupe en un monombre
       for group in input_vec {

           // le qutian est de 0, +
           let mut qutian_simplifer_du_group = fration::new(0, 1);
           for monombre in &group {
               qutian_simplifer_du_group = qutian_simplifer_du_group + monombre.qutian.clone();
           }
           output_vec.push(Monombre::new(
               qutian_simplifer_du_group,
               group[0].var_lettre,
           group[0].var_expoxen
           ))
       }
       output_vec
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

impl  std::fmt::Display for ElementAlgerbryque {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ElementAlgerbryque::EnumMonombre(Monombre) => write!(f, "{} ", Monombre),
            ElementAlgerbryque::Adition => write!(f, "+"),
            ElementAlgerbryque::Soutration => write!(f, "-"),
            ElementAlgerbryque::Divison => write!(f, "/"),
            ElementAlgerbryque::Mutiplucation => write!(f, "* "),
            ElementAlgerbryque::OuvertureDeParentaise => write!(f, "("),
            ElementAlgerbryque::FemeturDeParentaise => write!(f, ")"),
            ElementAlgerbryque::Egaliter => write!(f, "=")
        }

    }
}

impl  std::fmt::Display for Eqution {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for ElementAlgerbryque in &self.list_d_element {
            print!("{}", ElementAlgerbryque);
        }
        write!(f, "")
    }
}

impl ElementAlgerbryque {
    fn unwrap(&self) -> Monombre {
        match self {
            ElementAlgerbryque::EnumMonombre(Monombre) => Monombre.clone(),
            _ => panic!(),

        }
    }
}