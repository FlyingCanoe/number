use crossterm::*;
extern crate crossterm;


mod fration;
mod eqution_de_premier_degre;
mod eqution_de_second_degre;

fn main() {
    let terminal_input = input();
    let terminal = terminal();
    read_input_root(&terminal_input, &terminal);
}


fn read_input_root(terminal_inupt: &TerminalInput, terminal: &Terminal) {
    loop {
        terminal.set_size(60, 5);

        println!("entrer 1 pour résoudre une équation de premier degré");
        println!("entrer 2 pour résoudre une équation de second degré");
        println!("entrer 3 pour quitter le programme");

        let input = terminal_inupt.read_line();
        let str_input = match input  {
            Ok(str) => str,
            Err(_) => {
                println!("il y a une erreur");
                continue;
            }
        };

        if str_input == "1" {
            read_input_premier_degre(&terminal_inupt, terminal);
        }
        else if str_input == "2" {
            read_inupt_second_degre(&terminal_inupt, terminal);
        }
        else if str_input == "3" {
            terminal.exit();
        }
        else {
            println!("ne rentrer qu'un seul chiffre de 1 à 3")
        }
    }
}

fn read_input_premier_degre(terminal: &TerminalInput, terminal_cosmitque: &Terminal) {
    let _ = terminal_cosmitque.clear(ClearType::All);
    let a;
    let b;
    let y;

    //terminal_cosmitque .set_size(60, 4);

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
                println!("il y a une erreur, veuillez recommencer   | b = []");
                println!("                                          | y = []");
                continue
            }
        };
        a = match str_input.parse::<f64>() {
            Ok(a) => a,
            Err(_) => {
                terminal_cosmitque.clear(ClearType::All);
                println!("entrer la valeur de a dans ax+b = y       | a = []");
                println!("entrer un nombre                          | b = []");
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
                println!("il y a une erreur, veuilliez recommencer  | b = []");
                println!("                                          | y = []");
                continue
            }
        };
        b = match str_input.parse::<f64>() {
            Ok(b) => b,
            Err(_) => {
                terminal_cosmitque.clear(ClearType::All);
                println!("entrer la valeur de b dans ax+b = y       | a = {}", a);
                println!("n'entrer qu'un seul nombre                | b = []");
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
                println!("il y a une erreur, veuilliez recommencer   | b = {}", b);
                println!("                                          | y = []");
                continue
            }
        };
        y = match str_input.parse::<f64>() {
            Ok(y) => y,
            Err(_) => {
                terminal_cosmitque.clear(ClearType::All);
                println!("entrer la valeur de b dans ax+b = y       | a = {}", a);
                println!("veuillez svp entrer qu'un seul nombre     | b = {}", b);
                println!("                                          | y = []");
                continue
            }
        };
        break
    }


    eqution_de_premier_degre::solve_x(a, b, y, terminal_cosmitque, terminal);
    println!("appuyer sur enter pour continuer");

    terminal.read_line();
    let _ = terminal_cosmitque.clear(ClearType::All);
}

fn read_inupt_second_degre(terminal_input: &TerminalInput, terminal: &Terminal) {
    let a;
    let b;
    let c;

    terminal.set_size(100, 5);

    terminal.clear(ClearType::All);
    println!("entrer la valeur de a dans ax^2+bx+c = 0       | a = []");
    println!("                                               | b = []");
    println!("                                               | c = []");

    loop {
        let input = terminal_input.read_line();
        let str_input = match input {
            Ok(str) => str,
            Err(_) => {
                terminal.clear(ClearType::All);
                println!("entrer la valeur de a dans ax^2+bx+c = 0       | a = []");
                println!("il y a une erreur, veuillez recomencer         | b = []");
                println!("                                               | c = []");
                continue
            }
        };
        a = match str_input.parse::<isize>() {
            Ok(a) => a,
            Err(_) => {
                terminal.clear(ClearType::All);
                println!("entrer la valeur de a dans ax^2+bx+c = 0       | a = []");
                println!("n'entrer qu'un nombre entier                   | b = []");
                println!("                                               | c = []");
                continue
            }
        };
        break
    }
    terminal.clear(ClearType::All);
    println!("entrer la valeur de b dans ax^2+bx+c = 0      | a = {}", a);
    println!("                                              | b = []");
    println!("                                              | c = []");
    loop {
        let input = terminal_input.read_line();
        let str_input = match input {
            Ok(str) => str,
            Err(_) => {
                terminal.clear(ClearType::All);
                println!("entrer la valeur de b dans ax^2+bx+c = 0      | a = {}", a);
                println!("il y a une erreur, veuillez recomencer        | b = []");
                println!("                                              | c = []");
                continue
            }
        };
        b = match str_input.parse::<isize>() {
            Ok(b) => b,
            Err(_) => {
                terminal.clear(ClearType::All);
                println!("entrer la valeur de b dans ax^2+bx+c = 0      | a = {}", a);
                println!("n'entrer qu'un nombre entier                 | b = []");
                println!("                                              | c = []");
                continue
            }
        };
        break
    }
    terminal.clear(ClearType::All);
    println!("entrer la valeur de c dans ax^2+bx+c = 0      | a = {}", a);
    println!("                                              | b = {}", b);
    println!("                                              | c = []");

    loop {
        let input = terminal_input.read_line();
        let str_input = match input {
            Ok(str) => str,
            Err(_) => {
                terminal.clear(ClearType::All);
                println!("entrer la valeur de c dans ax^2+bx+c = 0      | a = {}", a);
                println!("il y a une erreur, veuillez recommencer       | b = {}", b);
                println!("                                              | c = []");
                continue
            }
        };
        c = match str_input.parse::<isize>() {
            Ok(c) => c,
            Err(_e) => {
                terminal.clear(ClearType::All);
                println!("entrer la valeur de c dans ax^2+bx+c = 0      | a = {}", a);
                println!("n'entrer qu'un nombre entier                  | b = {}", b);
                println!("                                              | c = []");
                continue
            }
        };
        break
    }

    eqution_de_second_degre::solve(a, b, c, terminal_input, terminal)
}

