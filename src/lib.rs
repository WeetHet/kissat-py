use pyo3::prelude::*;
use rustsat::{
    encodings::CollectClauses,
    instances::SatInstance,
    solvers::{ControlSignal, Solve, SolverResult, Terminate},
    types::{Clause, TernaryVal},
};
use std::collections::HashMap;
use std::{collections::hash_map::Entry, time::Instant};

/// Solves a Boolean satisfiability problem.
///
/// # Arguments
/// * `clauses` - A list of clauses, where each clause is a list of integers.
///   Positive integers represent positive literals, negative integers represent
///   negative literals.
/// * `time_limit` - Optional time limit in seconds after which the solver will terminate.
///
/// # Returns
/// A tuple containing:
/// * The first element is either:
///   - `Some(String)`: A solution string where '0' represents False, '1' represents True,
///     and '*' represents variables that can take either value (don't care).
///   - `None`: If no solution exists or the solver was interrupted.
/// * The second element is a boolean indicating whether the solver was interrupted (true)
///   or completed normally (false).
#[pyfunction]
fn solve_clauses(
    clauses: Vec<Vec<i32>>,
    time_limit: Option<f64>,
) -> PyResult<(Option<Vec<i32>>, bool)> {
    let mut instance = SatInstance::new();
    let mut lit_map = HashMap::new();

    let mut max_var = 0;
    let clauses: Vec<_> = clauses
        .into_iter()
        .map(|clause| {
            Clause::from_iter(clause.into_iter().map(|var| {
                let lit = match lit_map.entry(var.abs()) {
                    Entry::Vacant(entry) => *entry.insert(instance.new_lit()),
                    Entry::Occupied(entry) => *entry.get(),
                };
                max_var = max_var.max(var.abs());
                match var < 0 {
                    true => !lit,
                    false => lit,
                }
            }))
        })
        .collect();

    instance
        .extend_clauses(clauses)
        .expect("failed to extend clauses");

    let mut solver = rustsat_kissat::Kissat::default();

    let (cnf, _) = instance.into_cnf();
    solver.add_cnf(cnf).expect("failed to add cnf to solver");

    if let Some(time_limit) = time_limit {
        let start_time = Instant::now();
        solver.attach_terminator(move || {
            let elapsed = start_time.elapsed();
            match elapsed.as_secs_f64() >= time_limit {
                true => ControlSignal::Terminate,
                false => ControlSignal::Continue,
            }
        });
    }

    let result = solver.solve().expect("solver failed to execute");
    Ok(match result {
        SolverResult::Sat => {
            let model = solver.full_solution().expect("solver state incosistent");
            let mut solution = vec![];
            for var in 0..=max_var {
                if let Some(lit) = lit_map.get(&var) {
                    let assignment = model[lit.var()];
                    match assignment {
                        TernaryVal::True => solution.push(var),
                        TernaryVal::False => solution.push(-var),
                        TernaryVal::DontCare => {}
                    }
                }
            }

            (Some(solution), false)
        }
        SolverResult::Unsat => (None, false),
        SolverResult::Interrupted => (None, true),
    })
}

/// A Python module that provides access to the Kissat SAT solver from Python.
///
/// This module exposes functionality to solve Boolean satisfiability problems
/// using the Kissat solver, a state-of-the-art SAT solver implemented in Rust.
///
/// Functions:
///   - `solve_clauses`: Solves a SAT problem specified as a list of clauses.
///     Returns a solution if one exists, or None if the problem is unsatisfiable
///     or if the solver was interrupted due to a time limit.
#[pymodule]
fn kissat_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(solve_clauses, m)?)?;
    Ok(())
}
