


pub fn solve_x(a: f64, b: f64, mut y: f64) {

    //------//
    //etap 1//
    //------//
    // on affiche l'éqution initale


    //a est fait seulment partie de l'éqution, si il n'est pas égal a 1
    if is_not_one(a) {
        print!("a");
    }
    print!("x");

    // b fait seulment partie de l'éqution si il n'est pas égale a zero
    if is_not_nul(b) {
        print!("+b");
    }
    println!(" = y");

    //------//
    //etap 2//
    //------//
    //on renplace les variable et les inconue par leur valeur

    //si a n'est pas égale 1 <=> on afiche a
    if is_not_one(a) {
        print!("{}", a)
    }

    print!("x");

    if is_not_nul(b) {
        print!("+{}", b)
    }

    println!(" = {}", y);

    //------//
    //etap 3//
    //------//
    // on envoit b de l'autre bord de l'éqution pour qu'il se combine a la varable y


    // si b est nul il n'y a rien a munipuler a cette étap.
    if is_not_nul(b) {

        //si a n'est pas égale 1 <=> on afiche a
        if is_not_one(a) {
            print!("{}", a)
        }

        println!("x = {}-{}", y, b);

        y = y - b;
        drop(b);

        if is_not_one(a) {
            print!("{}", a)
        }
        println!("x = {}", y)
    }

    //------//
    //etap 4//
    //------//
    if is_not_one(a) {
        println!("x = {}", y);
        println!("    -------");
        println!("       {}", a);

        y = y / a;
        drop(a);
        println!("x = {}", y)
    }
}


/*pub fn solve_y(a: f64, mut x: f64, b: f64) {
    println!("ax+b = y");
    println!("{}*{}+{} = y", a, x, b);
    x = x * a;
    println!("{}+{} = y", x, b);
    x = x + b;
    println!("{} = y", x)
}*/

fn is_not_one(num: f64) -> bool { return num !=  1.0 }

fn is_not_nul(num: f64) -> bool {
    return num != 0.0
}