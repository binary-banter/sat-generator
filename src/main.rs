use itertools::Itertools;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Write;

const INPUT_COUNT: usize = 9;
const INSTRUCTION_COUNT: usize = 9;

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

// 00 center
// 01 a0
// 02 a1
// 03 a2
// 04 a3
// 05 a4
// 06 a5
// 07 a6
// 08 a7
// 09 a8
// 10 b0
// 11 a9
// 12 b1
// 13 aA
// 14 b2
// 15 at_least_one
// 16 more_than_one
// 17 two_or_three
// 18 center again

//    // stage 0
//     unsigned int a8;

//     unsigned int b0;

//     unsigned int a9;

//     unsigned int b1;

//
//     // stage 1
//     unsigned int aA;

//     unsigned int b2;

//
//     // magic phase
//     unsigned int at_least_one;

//     unsigned int more_than_one;

//     unsigned int two_or_three;

//     asm("lop3.b32 %0, %1, %2, %3, 0b11110110;" : "=r"(center) : "r"(center), "r"(aA), "r"(a9));
//     asm("lop3.b32 %0, %1, %2, %3, 0b01000000;" : "=r"(center) : "r"(center), "r"(two_or_three), "r"(more_than_one));

fn add_our_solution_clauses(sat: &mut Sat, truth_tables: &Vec<Vec<isize>>, connections: &Vec<[Vec<isize>; 3]>) {
    // asm("lop3.b32 %0, %1, %2, %3, 0b10010110;" : "=r"(a8) : "r"(a2), "r"(a1), "r"(a0));
    sat.add_clause([-truth_tables[0][7]]);
    sat.add_clause([ truth_tables[0][6]]);
    sat.add_clause([ truth_tables[0][5]]);
    sat.add_clause([-truth_tables[0][4]]);
    sat.add_clause([ truth_tables[0][3]]);
    sat.add_clause([-truth_tables[0][2]]);
    sat.add_clause([-truth_tables[0][1]]);
    sat.add_clause([ truth_tables[0][0]]);
    sat.add_clause([connections[0][2][1]]); // a0
    sat.add_clause([connections[0][1][2]]); // a1
    sat.add_clause([connections[0][0][3]]); // a2

    // asm("lop3.b32 %0, %1, %2, %3, 0b11101000;" : "=r"(b0) : "r"(a2), "r"(a1), "r"(a0));
    sat.add_clause([-truth_tables[1][7]]);
    sat.add_clause([-truth_tables[1][6]]);
    sat.add_clause([-truth_tables[1][5]]);
    sat.add_clause([ truth_tables[1][4]]);
    sat.add_clause([-truth_tables[1][3]]);
    sat.add_clause([ truth_tables[1][2]]);
    sat.add_clause([ truth_tables[1][1]]);
    sat.add_clause([ truth_tables[1][0]]);
    sat.add_clause([connections[1][2][1]]); // a0
    sat.add_clause([connections[1][1][2]]); // a1
    sat.add_clause([connections[1][0][3]]); // a2

    // asm("lop3.b32 %0, %1, %2, %3, 0b10010110;" : "=r"(a9) : "r"(a5), "r"(a4), "r"(a3));
    sat.add_clause([-truth_tables[2][7]]);
    sat.add_clause([ truth_tables[2][6]]);
    sat.add_clause([ truth_tables[2][5]]);
    sat.add_clause([-truth_tables[2][4]]);
    sat.add_clause([ truth_tables[2][3]]);
    sat.add_clause([-truth_tables[2][2]]);
    sat.add_clause([-truth_tables[2][1]]);
    sat.add_clause([ truth_tables[2][0]]);
    sat.add_clause([connections[2][2][4]]); // a3
    sat.add_clause([connections[2][1][5]]); // a4
    sat.add_clause([connections[2][0][6]]); // a5

    // asm("lop3.b32 %0, %1, %2, %3, 0b11101000;" : "=r"(b1) : "r"(a5), "r"(a4), "r"(a3));
    sat.add_clause([-truth_tables[3][7]]);
    sat.add_clause([-truth_tables[3][6]]);
    sat.add_clause([-truth_tables[3][5]]);
    sat.add_clause([ truth_tables[3][4]]);
    sat.add_clause([-truth_tables[3][3]]);
    sat.add_clause([ truth_tables[3][2]]);
    sat.add_clause([ truth_tables[3][1]]);
    sat.add_clause([ truth_tables[3][0]]);
    sat.add_clause([connections[3][2][4]]); // a3
    sat.add_clause([connections[3][1][5]]); // a4
    sat.add_clause([connections[3][0][6]]); // a5

    // asm("lop3.b32 %0, %1, %2, %3, 0b10010110;" : "=r"(aA) : "r"(a8), "r"(a7), "r"(a6));
    sat.add_clause([-truth_tables[4][7]]);
    sat.add_clause([ truth_tables[4][6]]);
    sat.add_clause([ truth_tables[4][5]]);
    sat.add_clause([-truth_tables[4][4]]);
    sat.add_clause([ truth_tables[4][3]]);
    sat.add_clause([-truth_tables[4][2]]);
    sat.add_clause([-truth_tables[4][1]]);
    sat.add_clause([ truth_tables[4][0]]);
    sat.add_clause([connections[4][2][7]]); // a6
    sat.add_clause([connections[4][1][8]]); // a7
    sat.add_clause([connections[4][0][9]]); // a8

    // asm("lop3.b32 %0, %1, %2, %3, 0b11101000;" : "=r"(b2) : "r"(a8), "r"(a7), "r"(a6));
    // sat.add_clause([-truth_tables[5][7]]);
    // sat.add_clause([-truth_tables[5][6]]);
    // sat.add_clause([-truth_tables[5][5]]);
    // sat.add_clause([ truth_tables[5][4]]);
    // sat.add_clause([-truth_tables[5][3]]);
    // sat.add_clause([ truth_tables[5][2]]);
    // sat.add_clause([ truth_tables[5][1]]);
    // sat.add_clause([ truth_tables[5][0]]);
    // sat.add_clause([connections[5][2][7]]); // a6
    // sat.add_clause([connections[5][1][8]]); // a7
    // sat.add_clause([connections[5][0][9]]); // a8

    // asm("lop3.b32 %0, %1, %2, %3, 0b11111110;" : "=r"(at_least_one) : "r"(b2), "r"(b1), "r"(b0));
    // sat.add_clause([-truth_tables[6][7]]);
    // sat.add_clause([ truth_tables[6][6]]);
    // sat.add_clause([ truth_tables[6][5]]);
    // sat.add_clause([ truth_tables[6][4]]);
    // sat.add_clause([ truth_tables[6][3]]);
    // sat.add_clause([ truth_tables[6][2]]);
    // sat.add_clause([ truth_tables[6][1]]);
    // sat.add_clause([ truth_tables[6][0]]);
    // sat.add_clause([connections[6][2][10]]); // b0
    // sat.add_clause([connections[6][1][12]]); // b1
    // sat.add_clause([connections[6][0][14]]); // b2

    // asm("lop3.b32 %0, %1, %2, %3, 0b11101000;" : "=r"(more_than_one) : "r"(b2), "r"(b1), "r"(b0));
    // sat.add_clause([-truth_tables[7][7]]);
    // sat.add_clause([-truth_tables[7][6]]);
    // sat.add_clause([-truth_tables[7][5]]);
    // sat.add_clause([ truth_tables[7][4]]);
    // sat.add_clause([-truth_tables[7][3]]);
    // sat.add_clause([ truth_tables[7][2]]);
    // sat.add_clause([ truth_tables[7][1]]);
    // sat.add_clause([ truth_tables[7][0]]);
    // sat.add_clause([connections[7][2][10]]); // b0
    // sat.add_clause([connections[7][1][12]]); // b1
    // sat.add_clause([connections[7][0][14]]); // b2

    // asm("lop3.b32 %0, %1, %2, %3, 0b01111000;" : "=r"(two_or_three) : "r"(at_least_one), "r"(aA), "r"(a9));
    // sat.add_clause([-truth_tables[8][7]]);
    // sat.add_clause([-truth_tables[8][6]]);
    // sat.add_clause([-truth_tables[8][5]]);
    // sat.add_clause([ truth_tables[8][4]]);
    // sat.add_clause([ truth_tables[8][3]]);
    // sat.add_clause([ truth_tables[8][2]]);
    // sat.add_clause([ truth_tables[8][1]]);
    // sat.add_clause([-truth_tables[8][0]]);
    // sat.add_clause([connections[8][2][11]]); // a9
    // sat.add_clause([connections[8][1][13]]); // aA
    // sat.add_clause([connections[8][0][15]]); // at_least_one
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

    //     for x in 1..(INPUT_COUNT + i) {
    //         for y in 0..=x {
    //             sat.add_clause([-connections[i][0][x], -connections[i][1][y]]);
    //         }
    //     }
    //
    //     for y in 1..(INPUT_COUNT + i) {
    //         for z in 0..=y {
    //             sat.add_clause([-connections[i][1][y], -connections[i][2][z]]);
    //         }
    //     }
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

    add_our_solution_clauses(&mut sat, &truth_tables, &connections);

    write!(f, "{}", sat).unwrap();
}

fn decode_output() {
    let input = include_str!("../output");
    let mut nums = input
        .split_whitespace()
        .map(|s| s.parse::<isize>().unwrap())
        .skip(2);

    for i in 0..INSTRUCTION_COUNT {
        print!("truth table {i}: ");
        for _ in 0..8 {
            print!("{}", (nums.next().unwrap() > 0) as u8)
        }
        println!();
        for j in 0..3 {
            print!("input {j}: ");
            for _ in 0..INPUT_COUNT + i {
                print!("{}", (nums.next().unwrap() > 0) as u8)
            }
            println!();
        }

        println!();
    }
}
