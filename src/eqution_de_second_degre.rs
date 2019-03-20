use num_rational::Rational;

pub fn solve(a: isize, b: isize, c: isize) {
    println!("{}x^2+{}x+{} = y", a, b, c);
    println!("mn  = ac");
    println!("m+n = b");
    let mn = a * c;
    println!("mn  = {}", mn);
    println!("m+n = {}", b);

    let mut m = 1;
    let mut n = 1;

    let v: Vec<(isize, isize)> = diviseur(mn);


    let mut mn_trouver = false;
    for i in v {
        if i.0 + i.1 == b {
            n = i.0;
            m = i.1;
            println!("m = {}", m);
            println!("n = {}", n);
            mn_trouver = true;
            break
        }
        else if i.0.wrapping_neg() + i.1 == b {
            n = i.0.wrapping_neg();
            m = i.1;
            println!("m = {}", m);
            println!("n = {}", n);
            mn_trouver = true;
            break
        }
        else if i.0 + i.1.wrapping_neg() == b {
            n = i.0;
            m = i.1.wrapping_neg();
            println!("m = {}", m);
            println!("n = {}", n);
            mn_trouver = true;
            break
        }
        else if i.0.wrapping_neg() + i.1.wrapping_neg() == b {
            n = i.0.wrapping_neg();
            m = i.1.wrapping_neg();
            println!("m = {}", m);
            println!("n = {}", n);
            mn_trouver = true;
            break
        }
    }
    if mn_trouver == false {
        println!("la résolution de ce trinombre est imposible");
        return;
    }
    println!("{}x^2+{}x+{}y+{} = y", a, m, n, c);
    let y1 = diviseur_comune(a, m);
    let y2 = diviseur_comune(n, c);
    println!("n = {}", n);
    println!("c = {}", c);

    let x1 = a/y1;
    let x2 = n/y2;
    let m1 = m/y1;
    let m2 = c/y2;
    println!("{}x({}x+{})+{}({}x+ {})", y1, x1, m1, y2, x2, m2 );
    println!("({}x+{})({}+{})", y1, y2, x1, m1);
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
    let mut number_final;
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