use crate::cnf::CNF;
use crate::game_of_life::generate_game_of_life_cnf;
use clap::Parser;
use std::io::{read_to_string, stdin};

mod cnf;
mod game_of_life;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    input_count: usize,
    instruction_count: usize,
    #[arg(short, long)]
    resolve: bool,
}

fn main() {
    let args = Args::parse();

    let game_of_life_cnf = generate_game_of_life_cnf(&args);

    if args.resolve {
        resolve(game_of_life_cnf);
        return;
    }

    println!("{game_of_life_cnf}");
}

fn resolve(cnf: CNF) {
    let assignments = read_to_string(stdin())
        .unwrap()
        .split_whitespace()
        .take(cnf.variable_count())
        .map(|assignment| !assignment.starts_with('-'))
        .collect::<Vec<_>>();

    for (variable, name) in cnf.names() {
        println!("{name} := {}", assignments[variable.index() - 1])
    }
}
