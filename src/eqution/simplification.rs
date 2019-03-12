use super::*;
use crate::fration;



pub fn run(eqution: &mut Equation) {
        eqution.simplification_des_exposent();

        eqution.simplification_par_aditont();
}

impl Equation {

    pub fn normalisation_de_eqution_entrant(&mut self) {
        /*
        la normalisation a la fonction d'elever certaine clase d'élement algebrique voici un
        list de ceux qui doive étre retirer

        monombre      : autoriser, il sont la base de l'éqution
        ouvetrure     : de parentaise : autoriser, leur retrait tien de la simplification, non de la normalisation
        fermeture     : de parentaise : autoriser pour les méme raison
        Egaliter      : autoriser si seul, si pluseur égaliter le programe plentra
        mutiplacation : je sais pas encore
        divison       : je ne sais pas encore
        aditon        : autoriser
        soutration    : non autoriser, 5x - 2X sera trensformer en 5x + -2X
*/
        for element_algebrique in self.list_d_element {

        }

    }

    //reduie les monombre qui non pas de variable et qui on un exposent superieur a 1
    pub fn simplification_des_exposent(&mut self) {
        for element_algebrique in &mut self.list_d_element {
            if let ElementAlgebrique::EnumMonombre(monombre) = element_algebrique {
                if let None = monombre.var_lettre {
                    if 1 != monombre.var_expoxen {
                        *element_algebrique = ElementAlgebrique::EnumMonombre(Monombre::new(
                            monombre.qutian.pusence(monombre.var_expoxen),
                            None,
                            1,
                        ))
                    }
                }
            }
        }
    }

    // rasenble les monombre en group que l'on peut redure en aditionant
    pub fn trie_pour_simplification_par_aditont(&self) -> Vec<Vec<Monombre>> {
        // une liste de groupe de monombre compatible
        let mut liste_groupe_de_monombre: Vec<Vec<Monombre>> = vec![];

        for element in &self.list_d_element {
            let mut group_deja_existen = false;

            //extrail les donner de l'élement pour les utiliser directement
            let monombre_de_element = match element {
                ElementAlgebrique::EnumMonombre(monombre) => monombre,
                ElementAlgebrique::Adition => continue,
                ElementAlgebrique::Egaliter => return liste_groupe_de_monombre,
                _ => panic!("la normalisation n'a pas été executer correctment, BUG!"),
            };

            for groupe in &mut liste_groupe_de_monombre {

                //pour savoir si une variable est compatible il faut savoir s'ils on la méme variable,
                // vu qu'un group on tous la méme variable, il sufi de prend un representan pour le verifer
                let representant = &groupe[0];


                if representant.compatible(&monombre_de_element) {
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

    pub fn simplification_par_aditont(&mut self) -> Vec<Monombre> {

        // un group est un ensemble de monombre pouven étre aditoner
        // pour former un seul monombre
        // ex
        // group 1 { 2x, 3x }
        // group 2 {1y, 2y}
        // group 3 {5}

        let list_de_group = self.trie_pour_simplification_par_aditont();

        let mut list_de_monombre_simplifer: Vec<Monombre> = vec![];

        //simplifi chaque groupe en un monombre
        for group in list_de_group {

            // le qutian est égale a la some des monombre. donc j'inislaise une varable qui a pour
            // valleur veut 0 et j'additone chaque monombre a cette varable
            let mut qutian_simplifer_du_group = fration::new(0, 1);


            for monombre in &group {
                qutian_simplifer_du_group = qutian_simplifer_du_group + monombre.qutian.clone();
            }
            list_de_monombre_simplifer.push(Monombre::new(
                qutian_simplifer_du_group,
                group[0].var_lettre,
                group[0].var_expoxen
            ))
        }
        return list_de_monombre_simplifer
    }
}
