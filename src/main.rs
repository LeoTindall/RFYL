extern crate rfyl;
extern crate time;

use std::{env, io, process};
use io::Write;
use rfyl::roll as roll;
use time::PreciseTime;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        let mut input_string = String::new();

        for (i, e) in args.iter().enumerate() {
            if i == 0 { continue; }
            input_string.push_str(e.as_ref());
        }

        let start = PreciseTime::now();
        let roll = roll(input_string);
        println!("------------------------------------------");        
        println!("Rolls:             {}", roll.get_rolls_string());
        println!("Formula:           {}", roll.get_formula_string_as_infix());
        println!("Rolls Formula:     {}", roll.get_rolls_formula_string_as_infix());
        println!("Result:            {}", roll.get_result());
        println!("------------------------------------------");
        let end = PreciseTime::now();        
        println!("Execution Time:    {}", start.to(end));
    } else {
        loop {
            let mut input = String::new();
            print!("> ");
            io::stdout().flush().expect("[Error] Flush failed!");        
            match io::stdin().read_line(&mut input) {
                Ok(n) => {
                    if n > 2 && input != "quit".to_string() {
                        let start = PreciseTime::now();
                        let roll = roll(input);
                        println!("------------------------------------------");
                        println!("Rolls:             {}", roll.get_rolls_string());
                        println!("Formula:           {}", roll.get_formula_string_as_infix());
                        println!("Rolls Formula:     {}", roll.get_rolls_formula_string_as_infix());
                        println!("Result:            {}", roll.get_result());
                        println!("------------------------------------------");
                        let end = PreciseTime::now();                        
                        println!("Execution Time:    {}", start.to(end));
                    } else if input == "quit".to_string() {
                        process::exit(0x0100);
                    } else {
                        println!("[Error] Please enter a formula.");
                    }
                },
                Err(e) => println!("[Error] Error while reading input: {}.", e)
            }
        }
    }
}