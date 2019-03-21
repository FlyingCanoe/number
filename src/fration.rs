/*
    use std::ops::Mul;
    use std::ops::Div;
    use std::ops::Add;
    use std::ops::Sub;


    #[derive(Debug, PartialEq, Copy)]
    pub struct Fration {
        pub nominateur: i64,
        pub denominateur: i64,

    }

    fn diviseur(number: i64) -> Vec<i64> {
        let mut vec: Vec<i64> = vec!();

        let number_final;
        if number.is_negative() == true {
            number_final = number.checked_neg().unwrap();
        }
        else {
            number_final = number;
        }

        for i in 1..number_final {
            let a = number as f64 / i as f64;

            if a.trunc() == a {
                vec.push(a.abs() as i64)
            }
        }
        vec
    }

    impl Fration {
        pub fn new(n: i64, d: i64) -> Fration {
        let mut new = Fration {
            nominateur: n,
            denominateur: d, 
        };
        new.simplifer();
        return new
        }

        pub fn negate(&mut self) {
            let (new_value, _) = self.nominateur.overflowing_neg();
            self.nominateur = new_value
        }
    }
    
    impl Clone for Fration {
        fn clone(&self) -> Fration {
            Fration {
                nominateur: self.nominateur,
                denominateur: self.denominateur,
            }
        }
    }


    impl Fration {
        pub fn simplifer(&mut self) {
            let list_diviseur_nominateur = diviseur(self.nominateur);
            let list_diviseur_denominateur = diviseur(self.denominateur);

            let mut plus_grand_diviseur_comun = 1; //asume 1 jusque qu'a temp que plus grand diviseur comun soit trouver 
                
            for diviseur_de_nominateur in list_diviseur_nominateur.iter() {
                    
                //regarde si le diviseur est ausi présent dans la liste de diviseur de denominateur
                for diviseur_de_denominateur in list_diviseur_denominateur.iter().rev() {
                        
                    if diviseur_de_nominateur == diviseur_de_denominateur {
                        plus_grand_diviseur_comun = *diviseur_de_nominateur; //un diviseur comun plus grand a été trouver, 
                        break                                               //d'autre pouron étre trouver dans les iteration future de la boucle
                    }
                }
            }

            // divise le nominateur et le denominateur par le plus grand diviseur comun
            self.nominateur   = self.nominateur/plus_grand_diviseur_comun;
            self.denominateur = self.denominateur/ plus_grand_diviseur_comun;
        }

        pub fn is_not_nul(&self) -> bool {
            return self.nominateur != 0
        }

        pub fn is_not_one(&self) -> bool {
            if self.nominateur == 1 {
                if self.denominateur == 1 {
                    return false
                }
                else { return true}
            }
            else { return true }

        }
    }

    impl Mul<Fration> for Fration {
        type Output = Fration;

    //  mutiply deux fration
        fn mul(self, mutiplieur: Fration) -> Fration {
            
            let mut produie = Fration {
                nominateur: self.nominateur * mutiplieur.nominateur,
                denominateur: self.denominateur * mutiplieur.denominateur,
            };
            produie.simplifer();
            return produie
        }
    }

    impl Div<Fration> for Fration {
        type Output = Fration;

        fn div(self, other: Fration) -> Fration  {
            let mut quotient = Fration {
                nominateur: self.nominateur * other.denominateur as i64,
                denominateur: self.denominateur * other.nominateur,
            };
            quotient.simplifer();
            return quotient
        }
    }

    impl Add<Fration> for Fration {
        type Output = Fration;

        fn add(self, other: Fration) -> Fration {
            let mut somme = Fration {
                nominateur: (self.nominateur * other.denominateur as i64) + 
                            (other.nominateur * self.denominateur as i64),

                denominateur: self.denominateur * other.denominateur,
            };
            somme.simplifer();
            return somme
        }
    }

    /*impl Add<Fration> for i64 {
        type Output = i64;

        fn add(self, other: Fration)
    }*/

    impl Sub<Fration> for Fration {
        type Output = Fration;

        fn sub(self, other: Fration) -> Fration {
            let mut difference = Fration {
                nominateur: (self.nominateur * other.denominateur as i64) - 
                            (other.nominateur * self.denominateur as i64),

                denominateur: self.denominateur * other.denominateur,
            };
            difference.simplifer();
            return difference

        }

    }

    impl std::fmt::Display for Fration {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            if self.denominateur == 1 {
                write!(f, "{}", self.nominateur)
            }
            else {
                write!(f, "({}/{})", self.nominateur, self.denominateur)
            }
        }
    }*/