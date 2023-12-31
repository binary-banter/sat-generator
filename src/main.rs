use crate::sat::CNF;
use clap::Parser;
use std::io::{read_to_string, stdin};

mod sat;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    resolve: bool,
}

fn main() {
    let args = Args::parse();

    let mut cnf = CNF::default();
    let x = cnf.new_variable();
    let y = cnf.new_named_variable("awesome_var");
    cnf.add_clause(x + y);
    cnf.add_clause(-x - y);

    if args.resolve {
        resolve(cnf);
        return;
    }

    println!("{cnf}");
}

fn resolve(cnf: CNF) {
    let assignments = read_to_string(stdin())
        .unwrap()
        .split_whitespace()
        .take(cnf.variable_count())
        .map(|assignment| assignment.starts_with('-'))
        .collect::<Vec<_>>();

    for (variable, name) in cnf.names() {
        println!("{name} := {}", assignments[variable.index() - 1])
    }
}
