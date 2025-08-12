use rustsat::instances::{SatInstance, Cnf};
use rustsat::lit;
use std::fs;
use std::io::BufReader;
pub fn load_cnf (path: &String) -> Result<(), Box<dyn std::error::Error>> {
    let instance: SatInstance = SatInstance::from_dimacs_path(&path)?;
    todo! ()
}
/*
fn load_cnfs(dir: &str) -> Vec<Cnf> {
    let mut cnfs = Vec::new();
    println!("Loading CNFs from {}", dir);
    let dir = Path::new(dir);
    if !dir.exists() || !dir.is_dir() {
        eprintln!("Directory {} does not exist.", dir.display());
        return cnfs;
    }
    for entry in dir.read_dir().expect("Failed to read directory") {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_file() {
            let file_path = entry.path();
            if file_path.extension().and_then(|s| s.to_str()) == Some("cnf") {
                match dimacs::parse_file(file_path.to_str().expect("Invalid UTF-8")) {
                    Ok(cnf) => cnfs.push(cnf),
                    Err(_) => eprintln!("Failed to parse file"),
                }
            }
        }
    }
    println!("Loaded {} CNF files.", cnfs.len());
    if cnfs.is_empty() {
        eprintln!(
            "Warning: No CNF files were loaded. Benchmarks might not run correctly. Check the \
            path pattern: {:?}",
            dir,
        );
    }
    cnfs
}
https://github.com/JacobJEdwards/rust_sat_solver/blob/master/benches/bench.rs
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
