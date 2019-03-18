use super::fration::Fration;

// ax+b
pub struct  EqutionPremierDegre {
}


pub fn solve_x(a: Fration, b: Fration, mut y: Fration) {

    //------//
    //etap 1//
    //------//
    // on affiche l'éqution initale


    //a est fait seulment partie de l'éqution, si il n'est pas égal a 1
    if a.is_not_one() {
        print!("a");
    }
    print!("x");

    // b fait seulment partie de l'éqution si il n'est pas égale a zero
    if b.is_not_nul() {
        print!("+b");
    }
    println!(" = y");

    //------//
    //etap 2//
    //------//
    //on renplace les variable et les inconue par leur valeur

    //si a n'est pas égale 1 <=> on afiche a
    if a.is_not_one() {
        print!("{}", a)
    }

    print!("x");

    if b.is_not_nul() {
        print!("+{}", b)
    }

    println!(" = {}", y);

    //------//
    //etap 3//
    //------//
    // on envoit b de l'autre bord de l'éqution pour qu'il se combine a la varable y


    // si b est nul il n'y a rien a munipuler a cette étap.
    if b.is_not_nul() {

        //si a n'est pas égale 1 <=> on afiche a
        if a.is_not_one() {
            print!("{}", a)
        }

        println!("x = {}-{}", y, b);

        y = y - b;
        drop(b);

        if a.is_not_one() {
            print!("{}", a)
        }
        println!("x = {}", y)
    }

    //------//
    //etap 4//
    //------//
    if a.is_not_one() {
        println!("x = {}", y);
        println!("    -------");
        println!("       {}", a);

        y = y / a;
        drop(a);
        println!("x = {}", y)
    }
}


pub fn solve_y(a: Fration, mut x: Fration, b: Fration) {
    println!("ax+b = y");
    println!("{}*{}+{} = y", a, x, b);
    x = x * a;
    println!("{}+{} = y", x, b);
    x = x + b;
    println!("{} = y", x)
}
