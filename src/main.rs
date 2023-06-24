use itertools::Itertools;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Write;

const INPUT_COUNT: usize = 9;
const INSTRUCTION_COUNT: usize = 10;

fn target_tt() -> Vec<bool> {
    to_truth_table::<INPUT_COUNT>(target)
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

struct Sat {
    // Nodes indexed from 1 ..= node_count
    node_count: isize,
    clauses: Vec<Vec<isize>>,
}

impl Sat {
    fn new() -> Self {
        Self {
            node_count: 2,
            clauses: vec![vec![1], vec![-2]],
        }
    }

    fn new_lit(&mut self) -> isize {
        self.node_count += 1;
        self.node_count
    }

    fn get_const(&self, b: bool) -> isize {
        if b {
            1
        } else {
            2
        }
    }

    fn add_clause(&mut self, c: impl IntoIterator<Item = isize>) {
        self.clauses.push(c.into_iter().collect())
    }
}

impl Display for Sat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "p cnf {} {}", self.node_count, self.clauses.len())?;

        for clause in &self.clauses {
            for thing in clause {
                write!(f, "{thing} ")?;
            }

            writeln!(f, "0")?;
        }

        Ok(())
    }
}

fn main() {
    create_cnf();
    // decode_output();
}

fn create_cnf() {
    let mut f = File::create("game_of_life.cnf").unwrap();

    let mut sat = Sat::new();

    let target_tt = target_tt();

    let mut truth_tables = vec![vec![]; INSTRUCTION_COUNT];
    let mut connections = Vec::new();
    for i in 0..INSTRUCTION_COUNT {
        connections.push([vec![], vec![], vec![]]);

        // make truth tables
        for _ in 0..8 {
            truth_tables[i].push(sat.new_lit());
        }

        // make connections
        // ... for inputs
        for side in 0..3 {
            for _ in 0..INPUT_COUNT {
                connections[i][side].push(sat.new_lit());
            }
            // ... for previous nodes
            for _ in 0..i {
                connections[i][side].push(sat.new_lit());
            }
            sat.add_clause(connections[i][side].iter().cloned());
            for pair in connections[i][side].iter().combinations(2) {
                sat.add_clause(pair.into_iter().map(|x| -x));
            }
        }

        for x in 1..(INPUT_COUNT + i) {
            for y in 0..=x {
                sat.add_clause([-connections[i][0][x], -connections[i][1][y]]);
            }
        }

        for y in 1..(INPUT_COUNT + i) {
            for z in 0..=y {
                sat.add_clause([-connections[i][1][y], -connections[i][2][z]]);
            }
        }
    }

    for input in 0..(1 << INPUT_COUNT) {
        let mut inputs = vec![];
        for i in 0..INPUT_COUNT {
            inputs.push(sat.get_const(input & (1 << i) != 0));
        }

        let mut outputs: Vec<isize> = vec![];

        for _ in 0..INSTRUCTION_COUNT - 1 {
            outputs.push(sat.new_lit());
        }
        let output = target_tt[input];
        outputs.push(sat.get_const(output));

        for i in 0..INSTRUCTION_COUNT {
            // Generate inputs
            let mut node_inputs = vec![];

            for side in 0..3 {
                let input_node = sat.new_lit();
                for j in 0..INPUT_COUNT {
                    // connections[i][side][j] -> (inputs[j] = input_node)
                    sat.add_clause([-connections[i][side][j], -inputs[j], input_node]);
                    sat.add_clause([-connections[i][side][j], inputs[j], -input_node]);
                }
                for j in 0..i {
                    // connections[i][side][INPUT_COUNT+j] -> (outputs[j] = input_node)
                    sat.add_clause([
                        -connections[i][side][INPUT_COUNT + j],
                        -outputs[j],
                        input_node,
                    ]);
                    sat.add_clause([
                        -connections[i][side][INPUT_COUNT + j],
                        outputs[j],
                        -input_node,
                    ]);
                }
                node_inputs.push(input_node);
            }

            //Generate output
            let output = outputs[i];

            //Drive output from inputs
            // ~A & ~B & ~C => (output = truth_tables[i][0])
            for j in 0..8 {
                let i1 = if j & (1 << 2) != 0 {
                    -node_inputs[0]
                } else {
                    node_inputs[0]
                };
                let i2 = if j & (1 << 1) != 0 {
                    -node_inputs[1]
                } else {
                    node_inputs[1]
                };
                let i3 = if j & (1 << 0) != 0 {
                    -node_inputs[2]
                } else {
                    node_inputs[2]
                };

                sat.add_clause([i1, i2, i3, output, -truth_tables[i][7 - j]]);
                sat.add_clause([i1, i2, i3, -output, truth_tables[i][7 - j]]);
            }
        }
    }

    write!(f, "{}", sat).unwrap();
}

fn decode_output() {
    let input = include_str!("../output");
    let mut nums = input
        .split_whitespace()
        .map(|s| s.parse::<isize>().unwrap())
        .skip(2);

    for i in 0..4 {
        print!("truth table {i}: ");
        for _ in 0..8 {
            print!("{}", (nums.next().unwrap() > 0) as u8)
        }
        println!();
        for j in 0..3 {
            print!("input {j}: ");
            for _ in 0..5 + i {
                print!("{}", (nums.next().unwrap() > 0) as u8)
            }
            println!();
        }

        println!();
    }
}
