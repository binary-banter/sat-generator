use crate::sat::CNF;

mod sat;

fn main() {
    let mut cnf = CNF::default();
    let x = cnf.new_variable();
    let y = cnf.new_named_variable("awesome_var");
    cnf.add_clause(x + y);
    cnf.add_clause(-x - y);
    println!("{cnf}");
}
