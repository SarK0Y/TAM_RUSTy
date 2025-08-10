use rustsat::instances::{SatInstance, Cnf};
use rustsat::lit;
use std::fs;
pub fn load_cnf (path: &String) -> Result<(), Box<dyn std::error::Error>> {
    let dimacs_str = fs::read_to_string( path )?;
    todo! ()
}
/*
use rustsat::instances::SatInstance;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load a DIMACS CNF file
    let dimacs_str = fs::read_to_string("example.cnf")?;
    let instance = SatInstance::from_dimacs(&dimacs_str)?;

    // Iterate over clauses
    for clause in instance.clauses() {
        println!("Clause: {:?}", clause);
    }

    Ok(())
}
use rustsat::instances::Cnf;
use rustsat::lit;

fn main() {
    let mut cnf = Cnf::new();
    cnf.add_clause(vec![lit![0], lit![1], !lit![2]]); // Example clause: (x0 ∨ x1 ∨ ¬x2)

    // Get all clauses
    for clause in cnf.clauses() {
        println!("Clause: {:?}", clause);
    }
}
use rustsat::instances::{SatInstance, Instance};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dimacs_str = fs::read_to_string("example.cnf")?;
    let instance = SatInstance::from_dimacs(&dimacs_str)?;

    match instance {
        Instance::Cnf { clauses, .. } => {
            println!("Total clauses: {}", clauses.len());
            for clause in clauses {
                println!("{:?}", clause);
            }
        }
        _ => println!("Not a CNF instance."),
    }

    Ok(())
}
use rustsat::clause;
let clause = clause![lit!(1), !lit!(2), lit!(3)]; // Same as above
use rustsat::instances::Cnf;

let mut cnf = Cnf::new();
cnf.add_clause(vec![lit!(1), !lit!(2), lit!(3)]); // Clause: (x1 ∨ ¬x2 ∨ x3)
use rustsat::instances::Cnf;
use std::fs;

fn main() -> std::io::Result<()> {
    // Create a CNF formula
    let mut cnf = Cnf::new();
    cnf.add_clause(vec![lit!(1), !lit!(2), lit!(3)]); // (x1 ∨ ¬x2 ∨ x3)
    cnf.add_clause(vec![!lit!(1), lit!(4)]);          // (¬x1 ∨ x4)

    // Convert to DIMACS format
    let dimacs_str = cnf.to_dimacs();

    // Write to a file
    fs::write("output.cnf", dimacs_str)?;

    println!("CNF saved to 'output.cnf'");
    Ok(())
}
use rustsat::instances::{SatInstance, Instance};
use std::fs;

fn main() -> std::io::Result<()> {
    // Load a DIMACS file (or create an instance programmatically)
    let dimacs_str = "p cnf 3 2\n1 -2 3 0\n-1 4 0\n";
    let instance = SatInstance::from_dimacs(dimacs_str).unwrap();

    // Save to file
    if let Instance::Cnf { clauses, .. } = instance {
        let cnf = Cnf::from_clauses(clauses);
        fs::write("output.cnf", cnf.to_dimacs())?;
    }

    Ok(())
}
*/
