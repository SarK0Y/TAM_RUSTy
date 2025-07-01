use once_cell::sync::Lazy;
 use Mademoiselle_Entropia::custom_traits::STRN;
#[derive(Clone, Debug, PartialEq)]
pub enum type_of_vars_expr {
    complex,
    simple,
    not,
}
pub enum token_status {
    too_large_indx,
    ret (String),
    empty,
    new_added
}
pub enum prime_token {
    semicolon,
    colon,
    dot,
    comma,
    paren (char),
    any_symb (char)
}
pub trait Alt_Assign {
    fn set (&mut self, new: Self);
}
impl Alt_Assign for usize {
    fn set (&mut self, new: Self) {
        *self = new;
    }
}
#[derive(Clone, Debug)]
pub struct found_local_vars {
    pub mut_or_not: Vec <bool>,
    pub pub_or_not: Vec <bool>,
    pub static_or_not: Vec <bool>,
    pub _type: Vec <String>,
    pub line: Vec <usize>,
    pub column: Vec <usize>,
    pub name: Vec <String>,
    pub txt: Vec <String>,
}
impl found_local_vars {
    pub fn new () -> Self {
        return Self {
            mut_or_not: Vec::<bool>::new(),
            pub_or_not: Vec::<bool>::new(),
            static_or_not: Vec::<bool>::new(),
            _type: Vec::<String>::new(),
            name: Vec::<String>::new(),
            txt: Vec::<String>::new(),
            line: Vec::<usize>::new(),
            column: Vec::<usize>::new(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct rExpr {
    pub txt: String,
    pub line: usize,
    pub column: usize,
    pub entry: usize,
    pub end: usize
}
impl rExpr {
   pub fn new () -> Self {
        return Self {
            txt: String::new(),
            line: 0,
            column: 0,
            entry: 0,
            end: 0
        }
    }
}
pub fn set_usize (set0: &mut usize, new: usize) {
    *set0 = new;
}
pub fn close_complex_var ( add_new_item: Option < &String >) -> Option <String > {
    static mut fin: Lazy <Vec <String> > = Lazy::new (|| {Vec::<String>::new ()});
    unsafe {
        if let Some (x) = add_new_item {
            fin.push (x.clone() );
        } return fin.pop ()
    }
}
pub fn log_name ( set: Option < &String >) -> Option <String > {
    static mut name: Lazy < String > = Lazy::new (|| {"/tmp/log_var".strn() });
    unsafe {
        if let Some (x) = set {
            *name = x.clone();
        } return Some ( name.clone() )
    }
}
