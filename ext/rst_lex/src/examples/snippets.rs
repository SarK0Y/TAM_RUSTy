/*
"Local { attrs: [], let_token: Let, pat: Pat::Ident { attrs: [], by_ref: None, mutability: None, ident: Ident { ident: \"tst\", span: #0 bytes(17083..17086) }, subpat: None }, init: Some(LocalInit { eq_token: Eq, expr: Expr::Lit { attrs: [], lit: Lit::Int { token: 411u32 } }, diverge: None }), semi_token: Semi }"
Loc "Local { attrs: [], let_token: Let, pat: Pat::Tuple { 
attrs: [], paren_token: Paren, elems: [
    Pat::Ident { attrs: [], by_ref: None, mutability: None, ident: Ident {
    ident: \"x\", span: #0 bytes(17106..17107) 
    }, subpat: None 
    }, Comma, Pat::Ident {
    attrs: [], by_ref: None, mutability: None, ident: Ident {
    ident: \"y\", span: #0 bytes(17109..17110) 
    }, subpat: None 
    }
    ]
    }, init: Some(LocalInit { 
    eq_token: Eq, expr: Expr::Tuple {
    attrs: [], paren_token: Paren, elems: [
    Expr::Lit { attrs: [], lit: Lit::Int {
    token: 47u64 
    } }, Comma, Expr::Lit { attrs: [], lit: Lit::Int { token: 357u32 } }] }, diverge: None }), semi_token: Semi }"
tst here
******************
Tokens:
These are the building blocks of the language, representing things like:
Identifiers: Names for variables, functions, etc. (e.g., x, myFunction). 
Keywords: Reserved words with specific meanings (e.g., if, while, for). 
Operators: Symbols that perform operations (e.g., +, -, *, =, /). 
Punctuation: Symbols like commas, semicolons, and parentheses. 
Literals: Constants, like numbers and strings (e.g., 123, "hello"). 

*/
