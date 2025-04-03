use std::{collections::HashMap, hash::Hash};
use std::cmp::min;
use std::io;
use rand::Rng;
use std::fmt;


const BOARD_WIDTH: u8 = 4;

#[derive(Eq, PartialEq, Hash, Debug)]
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

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s: String = self.pegs.iter().map(|c| match c {
                Color::Red => "R",
                Color::Green => "G",
                Color::Blue => "B",
                Color::Yellow => "Y",
                Color::Purple => "P",
                Color::Brown => "W"
            }).collect();

        write!(f, "{}", s)
    }
}

fn main() {
    println!("Welcome to Mastermind! How many guesses would you like?");
    let mut num_guesses = String::new();
    io::stdin().read_line(&mut num_guesses).expect("Failed to read number of guesses.");
    let num_guesses = num_guesses.trim().parse::<u32>().unwrap();
    println!("You have {} guesses!", num_guesses);

    let mut solution = Vec::new();
    let mut rng = rand::rng();

    for _ in 0..BOARD_WIDTH {
        let new_peg = match rng.random_range(0..6) {
            0 => Color::Red,
            1 => Color::Green,
            2 => Color::Blue,
            3 => Color::Yellow,
            4 => Color::Purple,
            5 => Color::Brown,
            _ => unreachable!()
        };
        solution.push(new_peg);
    }
    let solution_row = Row {pegs: solution};

    for _ in 0..num_guesses {
        println!("Input Format: R=Red, G=Green, B=Blue, Y=Yellow, P=Purple, W=Brown");
        println!("Input your guess:");
        let mut guess_text = String::new();
        io::stdin().read_line(&mut guess_text).expect("Error reading the guess");
        let guess_row = Row { pegs: guess_text.trim().chars().map(|c| match c {
            'R' | 'r' => Color::Red,
            'G' | 'g' => Color::Green,
            'B' | 'b' => Color::Blue,
            'Y' | 'y' => Color::Yellow,
            'P' | 'p' => Color::Purple,
            'W' | 'w' => Color::Brown,
            _ => unreachable!()
        }).collect() };
        let black_count = guess_row.black_peg(&solution_row);
        println!("black pegs: {black_count}");
        if black_count == BOARD_WIDTH as usize {
            println!("You win!");
            return;
        }
        let white_count = guess_row.white_peg(&solution_row);
        println!("white pegs: {white_count}");
    }
    println!("You've run out of guesses :(");
    println!("The correct solution is {}", solution_row);
    
}
