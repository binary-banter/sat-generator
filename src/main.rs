use crate::game_of_life::generate_game_of_life_cnf;
use clap::Parser;
use std::path::Path;
use std::{fs, io};

mod cnf;
mod game_of_life;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(default_value_t = 9)]
    input_count: usize,
    #[arg(default_value_t = 9)]
    instruction_count: usize,
    #[arg(short, long, value_name = "CNF")]
    resolve: Option<String>,
}

fn main() {
    let args = Args::parse();

    if let Some(cnf) = args.resolve {
        resolve(cnf);
        return;
    }

    println!("{}", generate_game_of_life_cnf(&args));
}

fn resolve(cnf: String) {
    let bindings = fs::read_to_string(Path::new(&cnf)).unwrap();
    let bindings = bindings.lines().take_while(|line| line.starts_with('c'));

    let assignments = io::read_to_string(io::stdin()).unwrap();
    let assignments = assignments.split_whitespace().collect::<Vec<_>>();

    for binding in bindings {
        let (name, index) = binding.split_once(" := ").unwrap();
        let assignment = !assignments[index.parse::<usize>().unwrap() - 1].starts_with('-');
        println!("{name} := {assignment}");
    }
}
