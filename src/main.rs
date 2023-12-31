use crate::cnf::CNF;
use crate::lut::LUT3;
use clap::Parser;
use std::io::{read_to_string, stdin};

mod cnf;
mod lut;

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

    let mut cnf = CNF::default();
    let mut luts = Vec::new();

    for index in 0..args.instruction_count {
        luts.push(LUT3::new(&mut cnf, index, &args));
    }

    // Add variables and constraints for every possible excitation to the network.
    for excitation in 0..1 << args.input_count {
        // Generate variables for every input.
        let mut inputs = Vec::new();
        for _ in 0..args.input_count {
            inputs.push(cnf.new_variable());
        }

        // Generate constraints so that the inputs match the excitation.
        for (i, input) in inputs.iter().cloned().enumerate() {
            if excitation & (1 << i) != 0 {
                cnf.add_clause(input);
            } else {
                cnf.add_clause(-input);
            }
        }

        // Generate variables for every intermediate result.
        let mut outputs = Vec::new();
        for _ in 0..args.instruction_count {
            outputs.push(cnf.new_variable());
        }

        // Generate constraint that the final output of the network is correct.
        if game_of_life_step(excitation) {
            cnf.add_clause(outputs[args.instruction_count - 1]);
        } else {
            cnf.add_clause(-outputs[args.instruction_count - 1]);
        }

        for i in 0..args.instruction_count {
            let lut_inputs = [cnf.new_variable(),cnf.new_variable(),cnf.new_variable()];
            luts[i].constrain_output(&mut cnf, lut_inputs, outputs[i]);
            luts[i].constrain_connections(&mut cnf, lut_inputs, &inputs, &outputs, &args)
        }
    }

    if args.resolve {
        resolve(cnf);
        return;
    }

    println!("{cnf}");
}

fn game_of_life_step(excitation: usize) -> bool {
    let count = (excitation & !1).count_ones();
    count == 3 || (excitation & 1 != 0 && count == 2)
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
