use std::fmt::{Display, Formatter};
use std::ops::{Add, Neg, Sub};

#[derive(Default)]
pub struct CNF<'p> {
    variables_count: usize,
    clauses: Vec<Clause>,
    names: Vec<(Variable, &'p str)>
}

#[derive(Default)]
pub struct Clause(Vec<Literal>);

#[derive(Copy, Clone)]
pub struct Variable(usize);

#[derive(Default)]
pub struct Literal(isize);

impl Display for CNF<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (variable, name) in &self.names {
            writeln!(f, "c {name} := {variable}")?;
        }

        writeln!(f, "p cnf {} {}", self.variables_count, self.clauses.len())?;

        for clause in &self.clauses {
            writeln!(f, "{clause}")?;
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

impl<'p> CNF<'p> {
    pub fn add_clause(&mut self, clause: impl Into<Clause>) {
        self.clauses.push(clause.into());
    }

    pub fn new_variable(&mut self) -> Variable {
        self.variables_count += 1;
        Variable::new(self.variables_count)
    }

    pub fn new_named_variable(&mut self, ident: &'p str) -> Variable {
        self.variables_count += 1;
        let variable = Variable::new(self.variables_count);
        self.names.push((variable, ident));
        variable
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

impl Add<Variable> for Literal{
    type Output = Clause;

    fn add(self, rhs: Variable) -> Self::Output {
        Clause(vec![self, rhs.into()])
    }
}

impl Sub for Variable{
    type Output = Clause;

    fn sub(self, rhs: Self) -> Self::Output {
        Clause(vec![self.into(), - rhs])
    }
}

impl Add for Variable{
    type Output = Clause;

    fn add(self, rhs: Self) -> Self::Output {
        Clause(vec![self.into(), rhs.into()])
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
