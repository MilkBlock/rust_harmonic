use rustsat::types::TernaryVal;
use rustsat::{instances::SatInstance, solvers::SolverResult};
use rustsat::solvers::*;

fn main() {
    println!("Hello, world!");
    let mut instance: SatInstance = SatInstance::new();
    let l1 = instance.new_lit();
    let l2 = instance.new_lit();
    instance.add_binary(l1, l2);
    instance.add_binary(!l1, l2);
    instance.add_unit(l1);
    println!("instance is {:?}",instance);
    let mut solver = rustsat_minisat::core::Minisat::default();
    solver.add_cnf(instance.into_cnf().0).unwrap();
    let res = solver.solve().unwrap();
    assert_eq!(res, SolverResult::Sat);
    let sol = solver.full_solution().unwrap();
    println!("solution of {:?} is {}",solver,sol);
    assert_eq!(sol[l1.var()], TernaryVal::True);
    assert_eq!(sol[l2.var()], TernaryVal::True);
}
