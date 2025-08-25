use rustsat::instances::{SatInstance, Cnf, ManageVars};
use rustsat::{lit, var};
use rustsat::types::{Lit, Var};
use rustsat::instances::ObjectVarManager;
use rustsat::encodings::am1::Encode;
use std::fs;
use std::path::Path;
use std::io::BufReader;
use crate::errMsg0;
use Mademoiselle_Entropia::{custom_traits::STRN, true_rnd::get_true_rnd_u64 as dice};

type _Status_for_vars = Vec <stats_for_var>;
type _CNF = Vec <Vec <Lit> >;
type _Naive_rank = Vec < (u32/*number of lits w/ given spin*/, usize/*var's indx*/, bool /*spin*/)>;
type _Map_vars = Vec <(bool /*Prime spin*/, Vec <usize> /*clauses w/ neg lit*/, Vec <usize> /*clauses w/ pos lit*/)>;
#[derive(PartialEq, Debug)]
pub enum order {
    unset,
    frozen,
}
#[derive(PartialEq, Debug)]
pub enum var_status {
    prime,
    neutral (u16)
}
pub struct stats_for_vars_n_clauses {
    pub vars: Vec <stats_for_var>,
    pub clauses: stats_for_clauses,
    pub rogue_vars: Vec <usize>
}
pub struct stats_for_var {
   // pub var_id: usize,
    pub spin: bool,
    pub solved_clauses: Vec <usize>,
    pub rogue_clauses:  Vec <usize>,
    pub vars_state: var_status,
    pub var_order: order,
}
impl stats_for_var {
    pub fn new () -> Self {
        return Self {
     //       var_id: 0,
            spin: true,
            solved_clauses: Vec::new(),
            rogue_clauses: Vec::new(),
            vars_state: var_status::neutral (0),
            var_order: order::unset,
        }
    }
}
pub struct clause_state {
    pub clause_id: usize,
    pub clause_keys: Vec <usize>,
    pub offset_in_clause: Vec <u16>
}
impl clause_state {
    pub fn new () -> Self {
        return Self {
            clause_id: 0,
            clause_keys: Vec::new(),
            offset_in_clause: Vec::new()
        }
    }
}
pub struct stats_for_clauses {
    pub solved_clauses: Vec <clause_state>,
    pub rogue_clauses: Vec <usize>,
    pub map: Vec<usize>
}
impl stats_for_clauses {
    pub fn new () -> Self {
        return Self {
            solved_clauses: Vec::new(),
            rogue_clauses: Vec::new(),
            map: Vec::new()
        }
    }
}
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
    let mut nr: _Naive_rank = _1st_look_rank (&mut _cnf, n_vars);
    let mut var_vals: Vec <bool> = simplest_attempt (&mut _cnf, &mut nr);
    if system_is_solved (&_cnf, &var_vals) { todo!() }
    search_w_details (&mut _cnf, &mut var_vals, &mut nr);
    dbg! (&n_vars);
    errMsg0 ("");
}
pub fn set_spins (v: &mut _Status_for_vars, nr: &_Naive_rank) {
    for n in nr {
        let id = n.1;
        let spin = n.2;
        v [id].spin = spin;
    }
}
pub fn _1st_look_rank (_cnf: &mut _CNF, n_vars: u32) -> Vec < (u32/*number of lits w/ given spin*/, usize/*var's indx*/, bool /*spin*/)>{
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
            naive_rank[j].0 /*rank*/ = var_share[j].1;
            naive_rank[j].2 /*spin*/ = true;
        } else { naive_rank[j].0 = var_share[j].0; }
    }
    naive_rank.sort_by (|a, b| {a.0.cmp (&b.0)});
    return naive_rank
}
pub fn simplest_attempt (cnf: &mut _CNF, nr: &mut _Naive_rank) -> Vec <bool> {
   let mut _1st_vals: Vec <bool> = Vec::with_capacity (nr.len() );
   for i in 0..nr.len() { _1st_vals.push (false); }
   for j in 0..nr.len() {
        let spin = nr[j].2;
        let var_id = nr[j].1;
        //let rank = nr[j].0;
        if spin { _1st_vals[var_id] = true; }
   } return _1st_vals
}
pub fn system_is_solved (_cnf: &_CNF, var_vals: &Vec <bool> ) -> bool {
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
pub fn search_w_details (_cnf: &mut _CNF, var_vals: &mut Vec <bool>, nr: &mut _Naive_rank ) {
    let mut details: stats_for_vars_n_clauses = solved_n_not_clauses_w_vars (&_cnf, &var_vals, &nr);
    set_spins (&mut details.vars, &nr);
    
}
pub fn eval_clause (clause: &Vec <Lit>, var_vals: &Vec <bool>) -> Option <clause_state> {
    let mut ret = clause_state::new();
    for k in 0..clause.len() {
        let _lit = &clause [k]; 
        let idx = _lit.var().idx ();
        if lit_val (_lit, var_vals [idx]) {
            ret.clause_keys.push (idx);
            ret.offset_in_clause.push (k as u16);
        }
    } 
    if ret.clause_keys.len () > 0 { return Some (ret) } return None
}
pub fn extra_eval_clause (clause: &Vec <Lit>, clause_id: usize, vars: &mut _Status_for_vars, var_vals: &Vec <bool>) -> Option <clause_state> {
    let mut ret = clause_state::new();
    let mut ids = Vec:: <usize> ::new();
    let mut rogue_ids = Vec:: <usize> ::new();
    for k in 0..clause.len() {
        let _lit = &clause [k]; 
        let idx = _lit.var().idx ();
        if lit_val (_lit, var_vals [idx]) {
            ret.clause_keys.push (idx);
            ret.offset_in_clause.push (k as u16);
            vars [idx].solved_clauses.push (clause_id);
            ids.push (idx);
            continue
        }
        rogue_ids.push (idx);
    }
    
    if clause.len () == rogue_ids.len () {
        for j in rogue_ids {
            vars [j].rogue_clauses.push (clause_id);
        }
    }
    _set_vars_status (vars, &ids);
    if ret.clause_keys.len () > 0 { return Some (ret) } return None
}
pub fn fast_eval_clause (clause: &Vec <Lit>, var_vals: &Vec <bool>) -> bool {
    let mut ret = true;
    for k in 0..clause.len() {
        let _lit = &clause [k]; //.clone();
        let idx = _lit.var().idx ();
        ret &= lit_val (_lit, var_vals [idx]);
    } return ret
}
pub fn solved_n_not_clauses (_cnf: &_CNF, var_vals: &Vec <bool>, nr: &_Naive_rank ) -> stats_for_clauses { // needs updates to be used
    let mut solved_n_not = stats_for_clauses::new();
    for i in 0.._cnf.len () {
        if let Some (mut x) = eval_clause (&_cnf [i], &var_vals) { 
            x.clause_id = i;
            solved_n_not.solved_clauses.push (x); continue 
        }
        solved_n_not.rogue_clauses.push (i);
    } return solved_n_not
}
pub fn solved_n_not_clauses_w_vars (_cnf: &_CNF, var_vals: &Vec <bool>, nr: &_Naive_rank ) -> stats_for_vars_n_clauses {
    let mut solved_n_not = stats_for_clauses::new();
    let mut vars: Vec <stats_for_var> = Vec::new();
    for _ in 0..var_vals.len() {vars.push (stats_for_var::new () );}
    for i in 0.._cnf.len () {
        if let Some (mut x) = extra_eval_clause (&_cnf [i], i, &mut vars, &var_vals) { 
            x.clause_id = i;
            solved_n_not.map.push (solved_n_not.solved_clauses.len() );
            solved_n_not.solved_clauses.push (x);
            continue 
        }
        solved_n_not.map.push (solved_n_not.rogue_clauses.len() );
        solved_n_not.rogue_clauses.push (i);
    } return stats_for_vars_n_clauses {
        vars: vars,
        clauses: solved_n_not,
        rogue_vars: Vec::new()
    }
}
pub fn set_vars_status (vars: &mut _Status_for_vars, var_ids: &Vec <usize>) {
    match var_ids.len () {
        0 => { return; },
        1 => { vars [var_ids [0] ].vars_state = var_status::prime; return },
        _ => {}
    }
    for j in var_ids {
        if vars [*j].vars_state == var_status::prime { continue }
        vars [*j].vars_state = var_status::neutral (var_ids.len () as u16 );
    }
}
pub fn _set_vars_status (vars: &mut _Status_for_vars, var_ids: &Vec <usize>) {
    match var_ids.len () {
        0 => { return; },
        1 => { vars [var_ids [0] ].vars_state = var_status::prime; return },
        _ => {}
    }
    for j in var_ids {
        match vars [*j].vars_state {
            var_status::prime => { continue },
            var_status::neutral (x) => {
                let len = var_ids.len () as u16;
                if len < x {
                    vars [*j].vars_state = var_status::neutral ( len )
                }
            }
        }
    }
}
pub fn dice_neutrals (info: &mut stats_for_vars_n_clauses) -> bool {
    let len = info.vars.len ();
    let mut rnd = Vec::<usize>::new();
    for _ in 0..8 {
        rnd.push (dice () as usize % len )
    }
    if let Some (v) = collect_neutral_vars (info, &rnd ) { rnd = v }
    return false
}
pub fn collect_neutral_vars (info: &stats_for_vars_n_clauses, selected: &Vec <usize>) -> Option <Vec <usize> > {
    let mut ret = Vec::<usize>::new ();
    let mut added = false;
    let sel_len = selected.len();
    for sel in selected {
        if info.vars [*sel].var_order == order::frozen {continue }
        if let var_status::neutral (_) = info.vars [*sel].vars_state {}
        else {continue }
        for id in *sel..sel_len {
            if info.vars [*sel].var_order == order::frozen {continue }
            if let var_status::neutral (_) = info.vars [*sel].vars_state {}
            else {continue }
            ret.push (id);
            added = true;
            break;
        }
        if added { continue }
        for id in *sel..0 {
            if info.vars [*sel].var_order == order::frozen {continue }
            if let var_status::neutral (_) = info.vars [*sel].vars_state {}
            else {continue }
            ret.push (id);
            break;
        }
        added = false;
    }
    if ret.len () > 0 { return Some (ret) }
    return None
}
pub fn flip_var (info: &mut stats_for_vars_n_clauses, var_vals: &mut Vec<bool>, var_id: usize, freeze: bool) {
    
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
*/
