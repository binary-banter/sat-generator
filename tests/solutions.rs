use std::fs::read_to_string;

struct LUT3 {
    inputs: [usize; 3],
    table: Vec<bool>,
}

impl LUT3 {
    fn compute(&self, state: &[bool]) -> bool {
        let index = ((state[self.inputs[2]] as usize) << 2)
            + ((state[self.inputs[1]] as usize) << 1)
            + (state[self.inputs[0]] as usize);

        self.table[index]
    }
}

fn game_of_life_step(excitation: usize) -> bool {
    let count = (excitation & !1).count_ones();
    count == 3 || (excitation & 1 != 0 && count == 2)
}

fn step_n_m(inputs: usize, instructions: usize) {
    let luts = parse(inputs, instructions);

    for excitation in 0..1 << inputs {
        let mut state = Vec::new();

        for input in 0..inputs {
            state.push(excitation & (1 << input) != 0)
        }

        for lut in &luts {
            state.push(lut.compute(&state))
        }

        assert_eq!(*state.last().unwrap(), game_of_life_step(excitation));
    }
}

fn parse(inputs: usize, instructions: usize) -> Vec<LUT3> {
    let solution = read_to_string(format!("tests/{inputs}_{instructions}")).unwrap();

    let mut luts = Vec::new();

    for line in solution.lines().skip(1) {
        let mut parts = line.split_whitespace().skip(1);

        let inputs = [
            parts.next().unwrap().parse().unwrap(),
            parts.next().unwrap().parse().unwrap(),
            parts.next().unwrap().parse().unwrap(),
        ];

        let table = parts.next().unwrap().chars().map(|c| c == '1').collect();

        luts.push(LUT3 { inputs, table });
    }

    luts
}

#[test]
fn step_4_3() {
    step_n_m(4, 3);
}

#[test]
fn step_5_4() {
    step_n_m(5, 4);
}

#[test]
fn step_6_6() {
    step_n_m(6, 6);
}

#[test]
fn step_7_7() {
    step_n_m(7, 7);
}

#[test]
fn step_8_8() {
    step_n_m(8, 8);
}
