use crate::cnf::{Variable, CNF, Clause};

pub struct LUT3{
    sides: Vec<Vec<Variable>>,
    table: Vec<Variable>
}

impl LUT3 {
    pub fn new(cnf: &mut CNF, index: usize, instructions: usize) -> Self {
        // Generate variables that indicate whether input `j` is connected to side `i`.
        let mut sides = Vec::new();
        for i in 0..3 {
            let mut side = Vec::new();

            // Note that we can only access inputs that come before this instruction.
            for j in 0..instructions + index {
                side.push(cnf.new_named_variable(format!("lut_{index}_side_{i}_connection_{j}")))
            }

            sides.push(side);
        }

        // Generate variables for the truth table of this instruction.
        let mut table = Vec::new();
        for i in 0..8 {
            table.push(cnf.new_named_variable(format!("lut_{index}_table_{i}")));
        }

        // Generate constraint that each side must have at least one connection.
        for side in &sides {
            cnf.add_clause(side.iter().cloned().sum::<Clause>());
        }

        Self{ sides, table }
    }

    pub fn with_io(&self, cnf: &mut CNF, inputs: [Variable; 3], output: Variable) {
        for (i, table_entry) in self.table.iter().cloned().enumerate() {
            let input_0 = if i & 1 == 0 { inputs[0].into() } else { -inputs[0] };
            let input_1 = if i & 2 == 0 { inputs[1].into() } else { -inputs[1] };
            let input_2 = if i & 4 == 0 { inputs[2].into() } else { -inputs[2] };

            cnf.add_clause(input_2 + input_1 + input_0 + output - table_entry);
            cnf.add_clause(input_2 + input_1 + input_0 - output + table_entry);
        }
    }
}
