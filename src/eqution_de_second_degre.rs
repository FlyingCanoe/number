use crossterm::*;

pub fn solve(a: isize, b: isize, c: isize, terminal_input: &TerminalInput, terminal: &Terminal) {
    terminal.clear(ClearType::All);
    terminal.set_size(100, 30);

    println!("appuyer sur la touche enter pour passer à la prochaine étape de la résolution");
    println!("{}x^2+{}x+{} = y", a, b, c);
    terminal_input.read_line();

    println!("a = {}", a);
    println!("b = {}", b);
    println!("c = {}", c);
    terminal_input.read_line();

    println!("mn  = ac");
    println!("m+n = b");
    terminal_input.read_line();

    println!("mn = {}*{}", a, c);
    terminal_input.read_line();

    let mn = a * c;
    println!("mn  = {}", mn);
    println!("m+n = {}", b);
    terminal_input.read_line();

    let mut m = 1;
    let mut n = 1;

    let v: Vec<(isize, isize)> = diviseur(mn);


    let mut mn_trouver = false;
    for i in v {
        if i.0 + i.1 == b {
            n = i.0;
            m = i.1;
            println!("note : la valeur de m et de n est réalisée par simple essais-erreurs il n'y a donc");
            println!("     : aucune démarche à afficher à cette étape");
            println!("m = {}", m);
            println!("n = {}", n);
            terminal_input.read_line();
            mn_trouver = true;
            break
        }
        else if i.0.wrapping_neg() + i.1 == b {
            n = i.0.wrapping_neg();
            m = i.1;
            println!("note : la valeur de m et de n est réalisée par simple essais-erreurs il n'y a donc");
            println!("     : aucune démarche à afficher à cette étape");
            println!("m = {}", m);
            println!("n = {}", n);
            terminal_input.read_line();
            mn_trouver = true;
            break
        }
        else if i.0 + i.1.wrapping_neg() == b {
            n = i.0;
            m = i.1.wrapping_neg();
            println!("note : la valeur de m et de n est réalisée par simple essais-erreurs il n'y a donc");
            println!("     : aucune démarche à afficher à cette étape");
            println!("m = {}", m);
            println!("n = {}", n);
            terminal_input.read_line();
            mn_trouver = true;
            break
        }
        else if i.0.wrapping_neg() + i.1.wrapping_neg() == b {
            n = i.0.wrapping_neg();
            m = i.1.wrapping_neg();
            println!("note : la valeur de m et de n est réalisée par simple essais-erreurs il n'y a donc");
            println!("     : aucune démarche à afficher à cette étape");
            println!("m = {}", m);
            println!("n = {}", n);
            terminal_input.read_line();
            mn_trouver = true;
            break
        }
    }
    if mn_trouver == false {
        println!("la résolution de ce trinombre est imposible, car aucune valeur de mn existe dans R");
        terminal_input.read_line();
        return;
    }
    println!("{}x^2+{}x+{}x+{} = y", a, m, n, c);
    terminal_input.read_line();
    let y1 = diviseur_comune(a, m);
    let y2 = diviseur_comune(n, c);

    let x1 = a/y1;
    let x2 = n/y2;
    let m1 = m/y1;
    let m2 = c/y2;
    println!("{}x({}x+{})+{}({}x+ {}) = 0", y1, x1, m1, y2, x2, m2 );
    terminal_input.read_line();
    println!("({}x+{})({}x+{}) = 0", y1, y2, x1, m1);
    terminal_input.read_line();
}


fn diviseur(number: isize) -> Vec<(isize, isize)> {
    let mut vec: Vec<(isize, isize)> = vec!();

    let number_final = number.wrapping_abs();

    for i in 1..number_final {
        let a = number as f64 / i as f64;

        if a.trunc() == a {
            let b = number /a as isize;
            vec.push((a as isize, b as isize))
        }
    }
    return vec
}

pub fn diviseur_comune(a: isize, b: isize) -> isize {
    let diviseur_a = diviseur_c(a);
    let diviseur_b = diviseur_c(b);


    for i in diviseur_a {
        for e in &diviseur_b {
            if i == *e {
                return *e;
            }
        }
    }
    return 1;
}

fn diviseur_c(num: isize) -> Vec<isize> {
    let mut vec: Vec<isize> = vec!();
    let number_final;
    if num.is_negative() == true {
        number_final = num.checked_neg().unwrap();
    }
    else {
        number_final = num;
    }

    for i in 1..number_final {
        let a = num as f64 / i as f64;

        if a.trunc() == a {
            vec.push(a.abs() as isize)
        }
    }
    vec
}