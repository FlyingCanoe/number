mod fration;

fn main() {

}

#[allow(non_camel_case_type)]
mod eqution_lineaire {
    use crate::fration::Fration;

    enum X {
        X,
    }

    enum Y {
        Y,
    }

    enum VariableIsoler {
        X,
        Y,
    }

    // aX+bY+c = 0 -les letre en majuscule ne sont pas incluse
    #[allow(non_camel_case_type)]
    struct Eqution_lineaire_Sous_forme_General {
        a: Fration,
        x: X,
        b: Fration,
        y: Y,
        c: Fration,
    }

    #[allow(non_camel_case_type)]
    struct Eqution_lineaire_Sous_forme_Fonctinel {
        a: Fration,
        x: X,
        c: Fration,
        valeur_isoler: VariableIsoler,
    }

    impl Eqution_lineaire_Sous_forme_General {

        /*
        ax+by+c = 0
        ax+c = by
        (ax+c)/b = y
        (ax/b)+(c/b) = y

        -pour isoler y if faux donc que: 

        a = a/b : x = x : c = c/b 
        */

        fn isoler_y(&self) -> Eqution_lineaire_Sous_forme_Fonctinel {
            Eqution_lineaire_Sous_forme_Fonctinel {
                a: self.a.clone() / self.b.clone(),
                x: X::X,
                c: self.c.clone() / self.b.clone(),
                valeur_isoler: VariableIsoler::Y

            }
        }

    }
}