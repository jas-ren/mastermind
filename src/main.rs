use std::{collections::HashMap, hash::Hash};
use std::cmp::min;
use std::io;


const BOARD_WIDTH: u8 = 4;

#[derive(Eq, PartialEq, Hash)]
enum Color {
    Red,
    Green,
    Blue,
    Yellow,
    Purple,
    Brown,
}


struct Row {
    pegs: Vec<Color>,
}

impl Row {
    fn check_position(&self, code: &Row) -> usize {
        let mut count = 0;
        for (i,_) in self.pegs.iter().enumerate() {
            if self.pegs[i] == code.pegs[i] {
                count += 1;
            }
        }
        count
    }

    fn check_color(&self, code: &Row) -> usize {
        // Returns the total number of correct colors, regardless of position correctness
        let mut self_counts = HashMap::new();

        for color in &self.pegs {
            *self_counts.entry(color).or_insert(0) += 1;
        }

        let mut code_counts = HashMap::new();
        for color in &code.pegs {
            *code_counts.entry(color).or_insert(0) += 1;
        }

        let mut overlap_count = 0;
        for (color, count) in self_counts {
            let code_count = code_counts.entry(color).or_insert(0);
            overlap_count += min(count, *code_count)
        }

        overlap_count
    }

    fn black_peg(&self, code: &Row) -> usize {
        self.check_position(code)
    }

    fn white_peg(&self, code: &Row) -> usize {
        self.check_color(code) - self.check_position(code)
    }
}

fn main() {
    println!("Welcome to Mastermind! How many guesses would you like?");
    let mut num_guesses = String::new();
    io::stdin().read_line(&mut num_guesses).expect("Failed to read number of guesses.");
    let num_guesses = u32::from_str_radix(&num_guesses, 10).unwrap();
    println!("You have {} guesses!", num_guesses);

    let solution = todo!();

    for i in 0..num_guesses {
        todo!()
    }
    
}
