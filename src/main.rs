mod fration;

fn main() {

}

mod eqution_lineaire {
    use crate::fration::Fration;

    //si le x ou le y est inconue
    enum X {
        VariableConue(u64),
        VariableInconue,
    }

    enum Y {
        VariableConue(u64),
        VariableInconue,
    }

    enum variable_isoler {
        x,
        y,
    }

    // aX+bY+c = 0 -les letre en majuscule ne sont pas incluse
    struct Eqution_lineaire_Sous_forme_General {
        a: Fration,
        x: X,
        b: Fration,
        y: Y,
        c: Fration,
    }

    struct Eqution_lineaire_Sous_forme_Fonctinel {
        a: Fration,
        x: X,
        b: Fration,
        c: Fration,
        valeur_isoler: variable_isoler,
    }

/*    impl Eqution_lineaire_Sous_forme_General {
        fn IsoleX(self) -> Eqution_lineaire_Sous_forme_Fonctinel {

        }
    }*/
}