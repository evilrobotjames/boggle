#![allow(dead_code)]
#[macro_use]
extern crate lazy_static;
use clap::Parser;
use game::grid::Grid;

mod dictionary;
mod game;

#[derive(Debug, Parser)]
struct Cli {
    #[arg(num_args(0..))]
    values: Vec<String>,
}

fn display_solution(grid: &mut Grid) {
    
    let mut words_printed = Vec::new();
    let mut length = usize::MAX;

    for word in grid.words_found() {

        // Anything less than three isn't really interesting.
        if word.len() <= 3 { 
            println!();
            return;
        }

        // Don't print words we've already printed.
        if words_printed.contains(word) {
            continue;
        } else {
            words_printed.push(word.to_string());
        }

        // Print a new heading, or the word.
        if word.len() < length {
            length = word.len();
            print!("\n{length}: {word} ")
        } else {
            print!("{word} ");
        }
    }
    println!("");
}

fn main() {

    let args = Cli::parse();
    let mut grid;

    // If given some values, create a game from them.  If not, create a random game to solve.
    if args.values.len() > 0 {
        let mut values = Vec::new();
        for value in args.values {
            values.push(value);
        }

        match game::new_from_values(values) {
            Err(err) => { eprintln!("Error: {}", err); std::process::exit(1) },
            Ok(g) => { grid = g; }
        }
    } else {
        grid = game::new_random();
    }

    println!("{}", grid);

    game::solve(&mut grid, dictionary::contains_word);
    
    display_solution(&mut grid);
}