extern crate num_rational;
extern crate crossterm;
use crossterm::{input, terminal, TerminalInput, ClearType, Terminal};



mod fration;
mod eqution_de_premier_degre;
mod eqution_de_second_degre;

fn main() {
    let terminal_cosmitique = terminal();
    let _ = terminal_cosmitique.clear(ClearType::All);
    let terminal = input();
    read_input_root(&terminal, &terminal_cosmitique)

}

fn read_input_root(terminal: &TerminalInput, terminal_cosmetique: &Terminal) {
    loop {
        //terminal_cosmitique.set_size(60,5);

        println!("entrer 1 pour resoud une eqution de premier degré");
        println!("entrer 2 pour resoud une eqution de second degré");
        println!("entrer 3 pour qutier le program");

        let input = terminal.read_line();
        let str_input = match input  {
            Ok(str) => str,
            Err(_) => {
                println!("il y a une erreur");
                continue;
            }
        };

        if str_input == "1" {
            read_input_premier_degre(&terminal, terminal_cosmetique);
        }
        else if str_input == "2" {
            read_inupt_second_degre(&terminal);
        }
        else if str_input == "3" {
            panic!()
        }
        else {
            println!("ne renter que un chifre de 1 à 3")
        }
    }
}

fn read_input_premier_degre(terminal: &TerminalInput, terminal_cosmituqe: &Terminal) {
    //let _ = terminal_cosmitique.clear(ClearType::All);
    let a;
    let b;
    let y;

    loop {
        println!("entrer la valeur de a dans ax+b = y");
        let input = terminal.read_line();
        let str_input = match input {
            Ok(str) => str,
            Err(_) => {
                println!("il y a une erreur, veulier recomencer");
                continue
            }
        };
        a = match str_input.parse::<f64>() {
            Ok(a) => a,
            Err(_) => {
                println!("n'entrer que un nombre");
                continue
            }
        };
        break
    }

    loop {
        println!("entrer la valeur de b dans ax+b = y");
        let input = terminal.read_line();
        let str_input = match input {
            Ok(str) => str,
            Err(_) => {
                println!("il y a une erreur, veulier recomencer");
                continue
            }
        };
        b = match str_input.parse::<f64>() {
            Ok(b) => b,
            Err(_) => {
                println!("n'entrer que un nombre");
                continue
            }
        };
        break
    }

    loop {
        println!("entrer la valeur de y dans ax+b = y");
        let input = terminal.read_line();
        let str_input = match input {
            Ok(str) => str,
            Err(_) => {
                println!("il y a une erreur, veulier recomencer");
                continue
            }
        };
        y = match str_input.parse::<f64>() {
            Ok(y) => y,
            Err(_) => {
                println!("n'entrer que un nombre");
                continue
            }
        };
        break
    }


    eqution_de_premier_degre::solve_x(a, b, y);
    println!("apuier sur enter pour contunuer");
    terminal.read_line();
    //let _ = terminal_cosmitique.clear(ClearType::All);
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

