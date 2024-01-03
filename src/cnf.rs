use itertools::Itertools;
use std::fmt::{Display, Formatter};
use std::iter::Sum;
use std::ops::{Add, Neg, Sub};

#[allow(clippy::upper_case_acronyms)]
#[derive(Default)]
pub struct CNF {
    variable_count: usize,
    clauses: Vec<Clause>,
    names: Vec<(Variable, String)>,
}

#[derive(Default)]
pub struct Clause(Vec<Literal>);

#[derive(Copy, Clone)]
pub struct Variable(usize);

#[derive(Default, Copy, Clone)]
pub struct Literal(isize);

impl Display for CNF {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (variable, name) in &self.names {
            writeln!(f, "c {name} := {variable}")?;
        }

        write!(f, "p cnf {} {}", self.variable_count, self.clauses.len())?;

        for clause in &self.clauses {
            write!(f, "\n{clause}")?;
        }

        Ok(())
    }
}

impl Display for Clause {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for literal in &self.0 {
            write!(f, "{literal} ")?;
        }
        write!(f, "0")
    }
}

impl Display for Variable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl CNF {
    pub fn add_clause(&mut self, clause: impl Into<Clause>) {
        self.clauses.push(clause.into());
    }

    pub fn new_variable(&mut self) -> Variable {
        self.variable_count += 1;
        Variable::new(self.variable_count)
    }

    pub fn new_named_variable(&mut self, ident: impl Into<String>) -> Variable {
        self.variable_count += 1;
        let variable = Variable::new(self.variable_count);
        self.names.push((variable, ident.into()));
        variable
    }

    pub fn exactly_once(&mut self, choices: &[Variable]) {
        if choices.len() < 6 {
            self.add_clause(choices.iter().cloned().sum::<Clause>());
            for (x, y) in choices.iter().cloned().tuple_combinations() {
                self.add_clause(-x - y);
            }
        }

        let commander = self.new_variable();
        self.add_clause(commander);
        self.exactly_once_aux(commander, choices);
    }

    fn exactly_once_aux(&mut self, commander: Variable, choices: &[Variable]) {
        if choices.len() <= 3 {
            // 1.
            for (x, y) in choices.iter().cloned().tuple_combinations() {
                self.add_clause(-x - y);
            }

            // 2.
            self.add_clause(choices.iter().cloned().sum::<Clause>() - commander);

            // 3.
            for x in choices.iter().cloned() {
                self.add_clause(commander - x);
            }

            return;
        }

        let d1 = choices.len() / 3;
        let d2 = 2 * choices.len() / 3;

        let commander_0 = self.new_variable();
        self.exactly_once_aux(commander_0, &choices[0..d1]);

        let commander_1 = self.new_variable();
        self.exactly_once_aux(commander_1, &choices[d1..d2]);

        let commander_2 = self.new_variable();
        self.exactly_once_aux(commander_2, &choices[d2..]);

        let choices = [commander_0, commander_1, commander_2];

        // 1.
        for (x, y) in choices.iter().cloned().tuple_combinations() {
            self.add_clause(-x - y);
        }

        // 2.
        self.add_clause(choices.iter().cloned().sum::<Clause>() - commander);

        // 3.
        for x in choices.iter().cloned() {
            self.add_clause(commander - x);
        }

        // 4.
        self.add_clause(-commander_0 - commander_1);
        self.add_clause(-commander_0 - commander_2);
        self.add_clause(-commander_1 - commander_2);
    }

    pub fn less_than_equal(&mut self, lhs_bits: &[Variable], rhs_bits: &[Variable]) {
        self.less_than_equal_aux(lhs_bits, rhs_bits);
    }

    fn less_than_equal_aux(&mut self, lhs_bits: &[Variable], rhs_bits: &[Variable]) -> Variable {
        let mut lhs_bits = Vec::from(lhs_bits);

        for _ in lhs_bits.len()..rhs_bits.len() {
            let temp = self.new_variable();
            lhs_bits.push(temp);
            self.add_clause(-temp);
        }

        let mut ts = Vec::new();

        let lhs_bits = lhs_bits.iter().rev().cloned().collect::<Vec<_>>();
        let rhs_bits = rhs_bits.iter().rev().cloned().collect::<Vec<_>>();

        for _ in 0..lhs_bits.len() {
            ts.push(self.new_variable());
        }

        // t_{i+1} => t_{i}
        for (first, second) in ts.iter().cloned().tuple_windows() {
            self.add_clause(-second + first);
        }

        for i in 0..ts.len() {
            // t_{i} => lhs_{i} = rhs_{i}
            self.add_clause(-ts[i] + lhs_bits[i] - rhs_bits[i]);
            self.add_clause(-ts[i] - lhs_bits[i] + rhs_bits[i]);
        }

        // t_{i-1} /\ -t_{i} => lhs_{i} = 0 /\ rhs_{i} = 1
        for i in 1..ts.len() {
            self.add_clause(-ts[i - 1] + ts[i] - lhs_bits[i]);
            self.add_clause(-ts[i - 1] + ts[i] + rhs_bits[i]);
        }

        self.add_clause(ts[0] - lhs_bits[0]);
        self.add_clause(ts[0] + rhs_bits[0]);

        ts.pop().unwrap()
    }

    pub fn less_than(&mut self, lhs_bits: &[Variable], rhs_bits: &[Variable]) {
        let t = self.less_than_equal_aux(lhs_bits, rhs_bits);

        // lhs != rhs
        self.add_clause(-t)
    }
}

impl Variable {
    fn new(index: usize) -> Self {
        Self(index)
    }
}

impl From<Variable> for Clause {
    fn from(value: Variable) -> Self {
        Self(vec![value.into()])
    }
}

impl From<Literal> for Clause {
    fn from(value: Literal) -> Self {
        Self(vec![value])
    }
}

impl From<Variable> for Literal {
    fn from(value: Variable) -> Self {
        Self(value.0 as isize)
    }
}

impl Add<Variable> for Literal {
    type Output = Clause;

    fn add(self, rhs: Variable) -> Self::Output {
        Clause(vec![self, rhs.into()])
    }
}

impl Sub for Variable {
    type Output = Clause;

    fn sub(self, rhs: Self) -> Self::Output {
        Clause(vec![self.into(), -rhs])
    }
}

impl Add for Variable {
    type Output = Clause;

    fn add(self, rhs: Self) -> Self::Output {
        Clause(vec![self.into(), rhs.into()])
    }
}

impl Add for Literal {
    type Output = Clause;

    fn add(self, rhs: Self) -> Self::Output {
        Clause(vec![self, rhs])
    }
}

impl Add<Variable> for Clause {
    type Output = Clause;

    fn add(mut self, rhs: Variable) -> Self::Output {
        self.0.push(rhs.into());
        self
    }
}

impl Sub<Variable> for Clause {
    type Output = Clause;

    fn sub(mut self, rhs: Variable) -> Self::Output {
        self.0.push(-rhs);
        self
    }
}

impl Add<Literal> for Clause {
    type Output = Clause;

    fn add(mut self, rhs: Literal) -> Self::Output {
        self.0.push(rhs);
        self
    }
}

impl Sub<Variable> for Literal {
    type Output = Clause;

    fn sub(self, rhs: Variable) -> Self::Output {
        Clause(vec![self, -rhs])
    }
}

impl Neg for Variable {
    type Output = Literal;

    fn neg(self) -> Self::Output {
        Literal(-(self.0 as isize))
    }
}

impl Sum<Variable> for Clause {
    fn sum<I: Iterator<Item = Variable>>(iter: I) -> Self {
        Self(iter.map(Into::into).collect())
    }
}

impl Sum<Literal> for Clause {
    fn sum<I: Iterator<Item = Literal>>(iter: I) -> Self {
        Self(iter.collect())
    }
}
