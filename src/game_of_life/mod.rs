use crate::cnf::{Clause, CNF};
use crate::game_of_life::lut::LUT3;
use crate::Args;
use itertools::Itertools;

mod lut;

pub fn generate_game_of_life_cnf(args: &Args) -> CNF {
    let mut cnf = CNF::default();
    let mut luts = Vec::new();

    for index in 0..args.instruction_count {
        luts.push(LUT3::new(&mut cnf, index, args));
    }

    // Prune: Center must be used.
    cnf.add_clause(
        luts.iter()
            .cartesian_product(0..3)
            .map(|(lut, side)| lut.input_node(side, 0).unwrap())
            .sum::<Clause>(),
    );

    // Prune: Last input must be used.
    cnf.add_clause(
        luts.iter()
            .cartesian_product(0..3)
            .map(|(lut, side)| lut.input_node(side, args.input_count - 1).unwrap())
            .sum::<Clause>(),
    );

    // Prune: Last instruction must use penultimate output.
    if args.instruction_count > 1 {
        cnf.add_clause(
            luts[args.instruction_count - 1]
                .input_node(2, args.input_count + args.instruction_count - 2)
                .unwrap(),
        );
    }

    // Prune: Tail ordering.
    for (first, second) in luts.iter().tuple_windows() {
        cnf.less_than_equal(first.side(2), second.side(2));
    }

    // Prune: Use inputs, except center, and intermediate results in order.
    // Idea: For every instruction that uses node index 'n', there must a previous instruction using node index 'n - 1'.
    for instruction in 0..args.instruction_count {
        // Use inputs, except center, in order.
        for input in 2..args.input_count {
            for side in 0..3 {
                // Node index `n`.
                let index = luts[instruction].input_node(side, input).unwrap();

                // Collect all node indices `n - 1` in previous (and current) instruction.
                let mut before = Vec::new();
                for lut in luts.iter().take(instruction + 1) {
                    for side_prev in 0..3 {
                        before.push(lut.input_node(side_prev, input - 1).unwrap());
                    }
                }

                cnf.add_clause(before.into_iter().sum::<Clause>() - index);
            }
        }

        // Use intermediate results in order.
        for output in args.input_count + 1..args.input_count + instruction {
            for side in 0..3 {
                // Node index `n`.
                let index = luts[instruction].input_node(side, output).unwrap();

                // Collect all node indices `n - 1` in previous (and current) instruction.
                let mut before = Vec::new();
                for prev_instr in (output - args.input_count + 1)..=instruction {
                    for side_prev in 0..3 {
                        before.push(luts[prev_instr].input_node(side_prev, output - 1).unwrap());
                    }
                }

                cnf.add_clause(before.into_iter().sum::<Clause>() - index);
            }
        }
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
