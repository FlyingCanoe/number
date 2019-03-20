extern crate num_rational;
extern crate crossterm;
use num_rational::Rational;
use crossterm::input;
use term::Terminal;
use std::iter::Skip;

mod fration;
mod eqution_de_premier_degre;
mod eqution_de_second_degre;

fn main() {
    let terminal = input();
    println!("entrer 1 pour resoud une eqution de premier degré");
    println!("entrer 2 pour resoud une eqution de second degré");
    println!("entrer 3 pour qutier le program");

    loop {
        input.read
    }
}

fn read_input_root(terminal: TerminalInput) {
    loop {
        let input = terminal.read_line();
        let str_input = match  {
            Ok(str) => str,
            Err(t) => {
                println!("il y a une erreur");
                Skip
            }
        };
    }
}

