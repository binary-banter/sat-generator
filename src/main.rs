use minisat::{Bool};

const INPUT_COUNT: usize = 9;
const INSTRUCTION_COUNT: usize = 4;

fn target_tt() -> Vec<bool> {
    to_truth_table::<9>(target)
}

fn target(inputs: u32) -> bool {
    let count = (inputs & !1).count_ones();
    count == 3 || (inputs & 1 != 0 && count == 2)
}

fn to_truth_table<const N: usize>(f: impl Fn(u32) -> bool) -> Vec<bool> {
    let mut array = Vec::new();
    for i in 0..1 << N {
        array.push(f(i));
    }
    array
}

fn main() {
    let mut sat = minisat::Solver::new();

    let target_tt = target_tt();

    let mut truth_tables = vec![vec![]; INSTRUCTION_COUNT];
    let mut connections = Vec::new();
    for i in 0..INSTRUCTION_COUNT{
        connections.push([vec![], vec![], vec![]]);

        // make truth tables
        for _ in 0..8 {
            truth_tables[i].push(sat.new_lit());
        }

        // make connections
        // ... for inputs
        for side in 0..3{
            for _ in 0..INPUT_COUNT {
                connections[i][side].push(sat.new_lit());
            }
            // ... for previous nodes
            for _ in 0..i {
                connections[i][side].push(sat.new_lit());
            }
            sat.assert_exactly_one(connections[i][side].iter().cloned());
        }
    }

    for input in 0..(1<<INPUT_COUNT) {
        let mut inputs = vec![];
        for i in 0..INPUT_COUNT{
            inputs.push(Bool::Const(input & (1 << i) != 0));
        }

        let mut outputs = vec![];
        for i in 0..INSTRUCTION_COUNT {
            // Generate inputs
            let mut node_inputs = vec![];

            for side in 0..3 {
                let input_node = sat.new_lit();
                for j in 0..INPUT_COUNT {
                    // connections[i][side][j] -> (inputs[j] = input_node)
                    let equiv = sat.equiv(inputs[j], input_node);
                    let impli = sat.implies(connections[i][side][j], equiv);
                    sat.equal(&impli, &Bool::Const(true));
                }
                for j in 0..i {
                    // connections[i][side][INPUT_COUNT+j] -> (outputs[j] = input_node)
                    let equiv = sat.equiv(outputs[j], input_node);
                    let impli = sat.implies(connections[i][side][INPUT_COUNT+j], equiv);
                    sat.equal(&impli, &Bool::Const(true));
                }
                node_inputs.push(input_node);
            }

            //Generate output
            let output = sat.new_lit();

            // ~A & ~B & ~C => (output = truth_tables[i][0])
            for j in 0..8 {
                let and = sat.and_literal([
                    if j & (1 << 0) == 0 { !node_inputs[0] } else { node_inputs[0] },
                    if j & (1 << 1) == 0 { !node_inputs[1] } else { node_inputs[1] },
                    if j & (1 << 2) == 0 { !node_inputs[2] } else { node_inputs[2] },
                ]);
                let equiv = sat.equiv(output, truth_tables[i][j]);
                let impli = sat.implies(and, equiv);
                sat.equal(&impli, &Bool::Const(true));
            }

            outputs.push(output);
        }


        let output = target_tt[input];
        sat.equal(outputs.last().unwrap(), &Bool::Const(output));
    }


    println!("starting model with #   vars = {}", sat.num_vars());
    println!("starting model with #clauses = {}", sat.num_clauses());

    let model = match sat.solve() {
        Ok(model) => model,
        Err(_) => {
            println!("UNSAT.");
            return;
        }
    };

    for i in 0..INSTRUCTION_COUNT {
        print!("{:2} - ", i + INPUT_COUNT);
        truth_tables[i].iter().map(|t| model.value(t)).for_each(|b| {
            print!("{}", b as u8);
        });
        print!(" ");
        for c in 0..3 {
            let connection = connections[i][c].iter().map(|t| model.value(t)).enumerate().filter(|(_, b)| *b).next().unwrap().0;
            print!("{:2} ", connection);
        }
        println!();
    }
}
