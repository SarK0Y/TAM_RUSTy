use rustsat::instances::{SatInstance, Cnf, ManageVars};
use rustsat::{lit, var};
use rustsat::types::{Lit, Var};
use rustsat::instances::ObjectVarManager;
use rustsat::encodings::am1::Encode;
use std::fs;
use std::path::Path;
use std::io::BufReader;
use crate::errMsg0;
type _CNF = Vec <Vec <Lit> >;
type _Naive_rank = Vec < (u32/*number of lits w/ given spin*/, usize/*lit indx*/, bool /*spin*/)>;
type _Map_vars = Vec <(bool /*Prime spin*/, Vec <usize> /*clauses w/ neg lit*/, Vec <usize> /*clauses w/ pos lit*/)>;
use Mademoiselle_Entropia::custom_traits::STRN;
pub fn load_cnf (path: &String) -> Result< (Cnf, u32), Box<dyn std::error::Error>> {
    let inst = SatInstance::<ObjectVarManager>::from_dimacs_path(&path)?;
    let n_vars: u32 = inst.n_vars();
    //dbg! (inst.n_lits() );
    let cnf: Cnf = inst.into_cnf().0;
    //cnf[0][0].clone();
    return Ok ( (cnf, n_vars ) )
}
pub fn try_to_solve_cnf (path: &String) {
    let mut path = path.replace ("try cnf", "").trim_end().trim_start ().strn ();
    path = "/home/mnt/usbhdd/gits/rustsat/data/AProVE11-12.cnf".strn();
    if let Ok ( x ) = path.parse:: <i64> () { path = crate::get_item_from_front_list( x, true); }
    let (mut cnf, n_vars ) = match load_cnf ( &path ) {
        Ok ((cnf, n_vars ) ) => { (cnf, n_vars ) },
        Err ( e ) => { crate::errMsg0 (&format! ("Sorry, Dear User, failed to load cnf file due to error {:?}", e)); return}
    };
  //  if cnf[0][0] == !lit! (0) {};
    let mut _cnf: Vec <Vec <Lit> > = Vec::new();
    for i in 0..cnf.len() {
        _cnf.push (Vec::new() );
        for j in 0..cnf[i].len() {
            _cnf[i].push (cnf [i] [j]);
            //_cnf [i][j] = !cnf [i][j];
        }
    }
    _1st_look_rank (&mut _cnf, n_vars);
    dbg! (&n_vars);
    errMsg0 ("");
}
pub fn _1st_look_rank (_cnf: &mut _CNF, n_vars: u32) -> Vec < (u32/*number of lits w/ given spin*/, usize/*lit indx*/, bool /*spin*/)>{
    let mut var_share: Vec < (u32 /*neg*/, u32 /*pos*/)> = Vec::new();
    for j in 0..n_vars as usize {
        var_share.push ( (0, 0) );
    }
    for i in 0.._cnf.len () {
        for k in 0.._cnf[i].len() {
            let val = _cnf [i] [k].clone();
            let idx = val.var().idx ();
            if val.is_neg () {var_share [idx].0 += 1;}
            else {var_share [idx].1 += 1;}
        }
    }
    let mut naive_rank: Vec <(u32, usize, bool)> = Vec::new();
    for j in 0..var_share.len() {
        naive_rank.push ((0, j, false));
        if var_share[j].0 < var_share[j].1 {
            naive_rank[j].0 = var_share[j].1;
            naive_rank[j].2 = true;
        } else { naive_rank[j].0 = var_share[j].0; }
    }
    naive_rank.sort_by (|a, b| {a.0.cmp (&b.0)});
    return naive_rank
}
pub fn simplest_attempt (cnf: &mut _CNF, nr: &mut _Naive_rank) {
   let mut _1st_vals: Vec <bool> = Vec::with_capacity (nr.len() );
   for i in 0..nr.len() { _1st_vals.push (false); }
   for j in 0..nr.len() {
        let spin = nr[j].2;
        let var_id = nr[j].1;
        //let rank = nr[j].0;
        if spin { _1st_vals[var_id] = true; }
   }
}
pub fn check_solution (_cnf: &_CNF, var_vals: Vec <bool> ) -> bool {
    let mut ret = true;
     for i in 0.._cnf.len () {
        for k in 0.._cnf[i].len() {
            let _lit = &_cnf [i] [k]; //.clone();
            let idx = _lit.var().idx ();
            ret &= lit_val (_lit, var_vals [idx]);
        } if !ret { return false }
    } return true
}
pub fn lit_val (_lit: &Lit, var_val: bool ) -> bool {
    /*if _lit.is_neg () && !var_val { return true } 
    if _lit.is_pos () && var_val { return true } 
    return false */
    return !(_lit.is_pos () ^ var_val )
}
pub fn search_w_details (_cnf: &_CNF, var_vals: Vec <bool>) {
    let mut map_vars: _Map_vars = Vec::new();
}
//fn
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
