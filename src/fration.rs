
    use std::ops::Mul;
    use std::ops::Div;
    use std::ops::Add;
    use std::ops::Sub;



    #[derive(Debug, PartialEq)]
    pub struct Fration {
        nominateur: i64,
        denominateur: i64,

    }

    fn diviseur(number: i64) -> Vec<i64> {
        let mut vec: Vec<i64> = vec!();

        for i in 2..number {
            let a = number as f64/i as f64;

            if a.trunc() == a {
                vec.push(a as i64)
            }
        }
    vec
    }

    impl Fration {
        fn new(n: i64, d: i64) -> Fration {
        let mut new = Fration {
            nominateur: n,
            denominateur: d, 
        };
        new.simplifer();
        return new
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
        fn simplifer(&mut self) {
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
            self.nominateur /= plus_grand_diviseur_comun;
            self.denominateur /= plus_grand_diviseur_comun; 
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
                denominateur: self.denominateur * other.denominateur,
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
