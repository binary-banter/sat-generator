use crate::cnf::{Clause, CNF};
use crate::game_of_life::lut::LUT3;
use crate::Args;

mod lut;

pub fn generate_game_of_life_cnf(args: &Args) -> CNF {
    let mut cnf = CNF::default();
    let mut luts = Vec::new();

    for index in 0..args.instruction_count {
        luts.push(LUT3::new(&mut cnf, index, args));
    }

    // Prune: All inputs and intermediate results must used at least once.
    for index in 0..args.input_count + args.instruction_count - 1 {
        let mut input_nodes = Vec::new();
        for lut in luts.iter().take(args.instruction_count) {
            for side in 0..3 {
                input_nodes.push(lut.input_node(side, index));
            }
        }
        cnf.add_clause(input_nodes.into_iter().flatten().sum::<Clause>())
    }

    // Add variables and constraints for every possible excitation to the network.
    for excitation in 0..1 << args.input_count {
        // Generate variables for every input and output.
        let mut state = Vec::new();
        for _ in 0..args.input_count + args.instruction_count {
            state.push(cnf.new_variable());
        }

        // Generate constraints so that the inputs match the excitation.
        for (i, input) in state.iter().cloned().enumerate().take(args.input_count) {
            if excitation & (1 << i) != 0 {
                cnf.add_clause(input);
            } else {
                cnf.add_clause(-input);
            }
        }

        // Generate constraint that the final output of the network is correct.
        let output = state.last().cloned().unwrap();
        if game_of_life_step(excitation) {
            cnf.add_clause(output)
        } else {
            cnf.add_clause(-output)
        }

        for i in 0..args.instruction_count {
            let lut_inputs = [cnf.new_variable(), cnf.new_variable(), cnf.new_variable()];
            luts[i].constrain_output(&mut cnf, lut_inputs, state[args.input_count + i]);
            luts[i].constrain_connections(&mut cnf, lut_inputs, &state)
        }
    }

    cnf
}

fn game_of_life_step(excitation: usize) -> bool {
    let count = (excitation & !1).count_ones();
    count == 3 || (excitation & 1 != 0 && count == 2)
}
