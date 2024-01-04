use crate::cnf::{Variable, CNF};
use crate::Args;

pub struct LUT3 {
    /// Nodes that connect to the input sides.
    input_nodes: Vec<Vec<Variable>>,
    /// Truth table used by the LUT3 instruction.
    pub(crate) table: Vec<Variable>,
}

impl LUT3 {
    pub fn new(cnf: &mut CNF, index: usize, args: &Args) -> Self {
        // Generate variables for input nodes. Multiple inputs are possible for each side.
        let mut input_nodes = Vec::new();
        for i in 0..3 {
            let mut side = Vec::new();

            // Note that we can only access inputs that come before this instruction.
            for j in 0..args.input_count + index {
                side.push(cnf.new_named_variable(format!("lut_{index}_side_{i}_connection_{j}")))
            }

            input_nodes.push(side);
        }

        // Generate variables for the truth table of this instruction.
        let mut table = Vec::new();
        for i in 0..8 {
            table.push(cnf.new_named_variable(format!("lut_{index}_table_{i}")));
        }

        // Generate constraint that each side must have exactly one connection.
        for side in &input_nodes {
            cnf.exactly_once(side)
        }

        // Prune: Input nodes must be ordered.
        cnf.less_than(&input_nodes[0], &input_nodes[1]);
        cnf.less_than(&input_nodes[1], &input_nodes[2]);

        Self { input_nodes, table }
    }

    pub fn constrain_connections(
        &self,
        cnf: &mut CNF,
        lut_inputs: [Variable; 3],
        state: &[Variable],
    ) {
        for (side, lup_input) in lut_inputs.into_iter().enumerate() {
            let index = self.input_nodes[0].len();
            for (i, input) in state.iter().cloned().enumerate().take(index) {
                let input_node = self.input_nodes[side][i];
                cnf.add_clause(-input_node + input - lup_input);
                cnf.add_clause(-input_node - input + lup_input);
            }
        }
    }

    pub fn constrain_output(&self, cnf: &mut CNF, inputs: [Variable; 3], output: Variable) {
        for (i, table_entry) in self.table.iter().cloned().enumerate() {
            let input_0 = if i & 1 == 0 { inputs[0].into() } else { -inputs[0] };
            let input_1 = if i & 2 == 0 { inputs[1].into() } else { -inputs[1] };
            let input_2 = if i & 4 == 0 { inputs[2].into() } else { -inputs[2] };

            cnf.add_clause(input_2 + input_1 + input_0 + output - table_entry);
            cnf.add_clause(input_2 + input_1 + input_0 - output + table_entry);
        }
    }

    pub fn input_node(&self, side: usize, index: usize) -> Option<Variable> {
        self.input_nodes[side].get(index).cloned()
    }

    pub fn side(&self, side: usize) -> &Vec<Variable> {
        &self.input_nodes[side]
    }
}
