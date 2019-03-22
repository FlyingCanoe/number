extern crate crossterm;
use crossterm::*;


mod fration;
mod eqution_de_premier_degre;
mod eqution_de_second_degre;

fn main() {
    let terminal_input = input();
    let terminal = terminal();
    read_input_root(&terminal_input, &terminal)
}


fn read_input_root(terminal_inupt: &TerminalInput, terminal_cosmetique: &Terminal) {
    loop {
        terminal_cosmetique.set_size(60, 5);

        println!("entrer 1 pour resoud une eqution de premier degré");
        println!("entrer 2 pour resoud une eqution de second degré");
        println!("entrer 3 pour qutier le program");

        let input = terminal_inupt.read_line();
        let str_input = match input  {
            Ok(str) => str,
            Err(_) => {
                println!("il y a une erreur");
                continue;
            }
        };

        if str_input == "1" {
            read_input_premier_degre(&terminal_inupt, terminal_cosmetique);
        }
        else if str_input == "2" {
            read_inupt_second_degre(&terminal_inupt);
        }
        else if str_input == "3" {
            terminal_cosmetique.exit();
        }
        else {
            println!("ne renter que un chifre de 1 à 3")
        }
    }
}

fn read_input_premier_degre(terminal: &TerminalInput, terminal_cosmitque: &Terminal) {
    let _ = terminal_cosmitque.clear(ClearType::All);
    let a;
    let b;
    let y;

    terminal_cosmitque .set_size(60, 4);

    println!("entrer la valeur de a dans ax+b = y       | a = []");
    println!("                                          | b = []");
    println!("                                          | y = []");

    loop {

        let input = terminal.read_line();
        let str_input = match input {
            Ok(str) => str,
            Err(_) => {
                terminal_cosmitque.clear(ClearType::All);
                println!("entrer la valeur de a dans ax+b = y       | a = []");
                println!("il y a une erreur, veulier recomencer     | b = []");
                println!("                                          | y = []");
                continue
            }
        };
        a = match str_input.parse::<f64>() {
            Ok(a) => a,
            Err(_) => {
                terminal_cosmitque.clear(ClearType::All);
                println!("entrer la valeur de a dans ax+b = y       | a = []");
                println!("veulier entrer un nombre                  | b = []");
                println!("                                          | y = []");
                continue
            }
        };
        break
    }

    terminal_cosmitque.clear(ClearType::All);
    println!("entrer la valeur de b dans ax+b = y       | a = {}", a);
    println!("                                          | b = []");
    println!("                                          | y = []");

    loop {
        let input = terminal.read_line();
        let str_input = match input {
            Ok(str) => str,
            Err(_) => {
                terminal_cosmitque.clear(ClearType::All);
                println!("entrer la valeur de b dans ax+b = y       | a = {}", a);
                println!("il y a une erreur, veulier recomencer     | b = []");
                println!("                                          | y = []");
                continue
            }
        };
        b = match str_input.parse::<f64>() {
            Ok(b) => b,
            Err(_) => {
                terminal_cosmitque.clear(ClearType::All);
                println!("entrer la valeur de b dans ax+b = y       | a = {}", a);
                println!("n'entrer que un nombre                    | b = []");
                println!("                                          | y = []");
                continue
            }
        };
        break
    }

    terminal_cosmitque.clear(ClearType::All);
    println!("entrer la valeur de y dans ax+b = y       | a = {}", a);
    println!("                                          | b = {}", b);
    println!("                                          | y = []");
    loop {
        let input = terminal.read_line();
        let str_input = match input {
            Ok(str) => str,
            Err(_) => {
                terminal_cosmitque.clear(ClearType::All);
                println!("entrer la valeur de b dans ax+b = y       | a = {}", a);
                println!("il y a une erreur, veulier recomencer     | b = {}", b);
                println!("                                          | y = []");
                continue
            }
        };
        y = match str_input.parse::<f64>() {
            Ok(y) => y,
            Err(_) => {
                terminal_cosmitque.clear(ClearType::All);
                println!("entrer la valeur de b dans ax+b = y       | a = {}", a);
                println!("veulier svp enter un nombre               | b = {}", b);
                println!("                                          | y = []");
                continue
            }
        };
        break
    }


    eqution_de_premier_degre::solve_x(a, b, y, terminal_cosmitque, terminal);
    println!("apuier sur enter pour contunuer");
    terminal.read_line();
    let _ = terminal_cosmitque.clear(ClearType::All);
}

fn read_inupt_second_degre(terminal: &TerminalInput) {
    let a;
    let b;
    let c;

    loop {
        println!("entrer la valeur de a dans ax^2+bx+c = 0");
        let input = terminal.read_line();
        let str_input = match input {
            Ok(str) => str,
            Err(_) => {
                println!("il y a une erreur, veulier recomencer");
                continue
            }
        };
        a = match str_input.parse::<isize>() {
            Ok(y) => y,
            Err(_) => {
                println!("n'entrer que un nombre entier");
                continue
            }
        };
        break
    }

    loop {
        println!("entrer la valeur de b dans ax^2+bx+c = 0");
        let input = terminal.read_line();
        let str_input = match input {
            Ok(str) => str,
            Err(_) => {
                println!("il y a une erreur, veulier recomencer");
                continue
            }
        };
        b = match str_input.parse::<isize>() {
            Ok(b) => b,
            Err(_) => {
                println!("n'entrer que un nombre entier");
                continue
            }
        };
        break
    }

    loop {
        println!("entrer la valeur de c dans ax^2+bx+c = 0");
        let input = terminal.read_line();
        let str_input = match input {
            Ok(str) => str,
            Err(_) => {
                println!("il y a une erreur, veulier recomencer");
                continue
            }
        };
        c = match str_input.parse::<isize>() {
            Ok(c) => c,
            Err(_e) => {
                println!("n'entrer que un nombre entier");
                continue
            }
        };
        break
    }

    eqution_de_second_degre::solve(a, b, c)
}

