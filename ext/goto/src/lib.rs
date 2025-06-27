#![allow(static_mut_refs)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(non_upper_case_globals)]
//mod goto;
//pub use crate::goto::{label, goto};
use rst_lex::lex::{collect_not_nested_let_tokens, rExpr, leave_file_mark, token_for_loop, get_lines_in_fn, collect_all_assigns};
use rst_lex::edit_funx as edit;
use substring::Substring;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse::{Parse, ParseStream, Result}, parse_quote, ItemFn, LitStr, Stmt, Meta, MetaList, MetaNameValue, punctuated::Punctuated, Attribute,
token::Comma, Expr, Lit, Token, PatIdent, Pat, Local, PathSegment, DeriveInput};
use proc_macro2::{TokenStream as TokenStream2, Span};
use Mademoiselle_Entropia::custom_traits::STRN;
struct AttrArgs {
    metas: Punctuated<Meta, Token![,]>,
}
impl Default for AttrArgs {
    fn default() -> Self {
        Self {
            metas: Punctuated::new(),
        }
    }
}
struct AttrArgsIter<'a> {
    iter: syn::punctuated::Iter<'a, Meta>, // Use a slice iterator for the Punctuated collection
}

impl<'a> Iterator for AttrArgsIter<'a> {
    type Item = &'a Meta; // Yield references to Meta items

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next() // Call the next method on the slice iterator
    }
}

// Implement a method to return the iterator
impl AttrArgs {
    fn iter(&self) -> AttrArgsIter {
        AttrArgsIter {
            iter: self.metas.iter(), // Create an iterator from the Punctuated collection
        }
    }
}
impl Parse for AttrArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let metas = Punctuated::<Meta, Token![,]>::parse_terminated_with(input, Meta::parse)?;
        Ok(AttrArgs { metas })
    }
}
   macro_rules! _prnt_func {
    ($name:ident, $body:block) => {
        pub fn $name() {
            println!("{}", stringify!($body));
        }
    };
}
#[proc_macro_attribute]
pub fn inject(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input: ItemFn = parse_macro_input!(item as ItemFn);
    let mut new_stmts = Vec::<Stmt>::new(); 
    let strn_stmts = format! ("{:?}", input.block.stmts);
    let add_to: Stmt = syn::parse_quote! {
                println!("yst {}", #strn_stmts);
            };
    let stmts = &input.block.stmts;
    let stmts_len = stmts.len();
    for j in 0..stmts_len {
        if j == stmts_len -1 {new_stmts.push ( add_to.clone() ); }
        new_stmts.push(stmts [j].clone());
    }            
    input.block.stmts = new_stmts;
    return TokenStream::from(quote! { #input })
}
#[proc_macro_attribute]
pub fn inject_after_hello(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input: ItemFn = parse_macro_input!(item as ItemFn);
    let mut new_stmts = Vec::<Stmt>::new(); 

    for stmt in input.block.stmts {
        // Push the original statement
        new_stmts.push(stmt.clone());

        // Check if it's the specific println! macro invocation
        if let Stmt::Expr(syn::Expr::Macro(expr_macro), _) = &stmt {
            let macro_path = &expr_macro.mac.path.segments;
            let macro_tokens = expr_macro.mac.tokens.to_string();
            if macro_path.len() == 1
                && macro_path[0].ident == "println"
                && macro_tokens.contains("\"hello\"")
            {
                // Inject code after the matched println!
                new_stmts.push(syn::parse_quote! {
                    println!("-- injected after Hello, world!");
                });
            }
        }
    }

    input.block.stmts = new_stmts;

    TokenStream::from(quote! { #input })
}
//#![feature(trace_macros)]
//trace_macros!(true);
#[proc_macro_attribute]
pub fn inject_tst(args: TokenStream, item: TokenStream) -> TokenStream {
   let (sub_fn, sub_fn_id) = prnt_func ("sub_fn", item.clone());
    let item_str = format! ("{}", item.clone() );
    let mut ext_quote = quote!();
    let item0 = item.clone();
    let mut input: ItemFn = parse_macro_input!(item0 as ItemFn);
    let lit: LitStr = LitStr::new("Alice", proc_macro2::Span::call_site());
    let val = Expr::Lit(syn::ExprLit {
        attrs: vec![],
        lit: Lit::Str(lit),
    });
    let meta_name_value = MetaNameValue {
        path: syn::parse_quote!(name), // The name of the attribute
        eq_token: Default::default(),   // The '=' token
        value: val,
    };
   /* let mut args_meta: Meta = Meta::NameValue(meta_name_value);
    let mut stop_err = false;
    match syn::parse::<Meta>(args) {
        Ok(meta) => { args_meta = meta; },
        Err(e) => { let err = format! ("customized compiler's error {:#?}", e); stop_err = true;
                    return TokenStream::from (quote! {
                        pub fn lets_prnt_func () {
                            println! ("{}", #err);
                        }
                    })},
    };*/
    let args = parse_macro_input! (args as AttrArgs);
    //if stop_err {compile_error! ("stop_err"); }
    for arg in args.iter() {
        match arg {
            Meta::Path(path) => { ext_quote.extend (
                quote! { println!("Simple attribute: {:?}", #path); }
                );
            }
            Meta::List(list) => { 
                let list = list.tokens.clone();
            ext_quote.extend (
                quote! {println!("Attribute with args: {:?}", #list); }
                );
            }
            Meta::NameValue(name_value) => { 
                let ident = name_value.path.get_ident().unwrap().to_string();
                let value = name_value.value.clone();
            ext_quote.extend (
            quote! { println!("Name-value attribute: {} = {:?}", 
                    #ident,
                    #value);
                } );
            }
        };
    }
    //let attrss = &attr_in.attrs;
 //   println!("tssst {} val {}", _attrs.to_string(), "42");
  /*for attr in attrss {
    println!("enter to attrs", );
    if !attr.path().is_ident("tst00") {
        match &attr.meta {
            Meta::Path(_) => {
                // Just #[my_attr]
            }
            Meta::List(list) => {
                // #[my_attr(arg1, arg2)]
                let nested = list.parse_args_with(Punctuated::<Meta, Comma>::parse_terminated);
                
                match nested {
                    Ok(nested) => {
                        for meta in nested {
                            match meta {
                                Meta::Path(path) => println!("Path: {:?}", path),
                                Meta::List(list) => println!("Nested list: {:?}", list),
                                Meta::NameValue(nv) => {
                                    println!("Name-value: {} = {:#?}", 
                                        nv.path.get_ident().unwrap(), 
                                        nv.value
                                    );
                                }
                            }
                        }
                    }
                    Err(e) => {
                        // Handle parse error
                        return TokenStream::from(e.to_compile_error());
                    }
                }
            }
            Meta::NameValue(nv) => {
                // #[my_attr = value]
                println!("Attribute value: {:?}", nv.value);
            }
        }
    }
}*/

    let input_str = format! ("{:#?}", input);
    let mut new_stmts = Vec::<Stmt>::new(); 
    let strn_stmts = format! ("{:?}", input.block.stmts);
    let add_to: Stmt = syn::parse_quote! {
                println!("yst {}\n{}\n{}", #strn_stmts, #input_str, #item_str);
            };
    let stmts = &input.block.stmts;
    let stmts_len = stmts.len();
    for j in 0..stmts_len {
        if j == stmts_len -1 {new_stmts.push ( add_to.clone() ); }
        new_stmts.push(stmts [j].clone());
    }            
    input.block.stmts = new_stmts; 
    let input_str = format! ("{:#?}", input);
    let ext_quote_str = ext_quote.to_string();
    //let sub_fn: Stmt = parse_quote! (sub_fn as Stmt);
    return TokenStream::from(quote! {
        #sub_fn
        pub fn lets_prnt_func () {
            println! ("input {} ext {}", #input_str, #ext_quote_str );
            #ext_quote
            #sub_fn_id();
            }} )
}
#[proc_macro]
pub fn my_macro(input: TokenStream) -> TokenStream {
    // Create a new identifier with a specific span
    let ident = syn::Ident::new("my_variable", Span::call_site());
    let mut new_stmts = Vec::<Stmt>::new(); 
    let mut input: ItemFn = parse_macro_input!(input as ItemFn);
    // Generate code that uses the identifier
    let expanded = TokenStream::from (quote! {
        let #ident = #input;
    } );

    expanded.into()
}
  #[proc_macro]
   pub fn stmt_to_tokenstream(input: TokenStream) -> TokenStream {
       // Parse the input TokenStream into a Stmt
       let stmt: Stmt = parse_macro_input!(input as Stmt);

       // Convert the Stmt back to a TokenStream
       let output = quote! {
           #stmt
       };

       output.into()
   }
   #[proc_macro_attribute]
   pub fn just_prnt (args: TokenStream, input: TokenStream) -> TokenStream {
        let func_body = input.to_string();
        let out = quote! {
            pub fn lets_prnt_func () {
                println! ("{}", #func_body);
            }
        };
        return out.into()
   }
#[proc_macro_attribute]
pub fn prnt_vars(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut  func_body = item.to_string ();
    let func_body_last_exit = func_body.substring (0, func_body.chars().count() - 1);
    let mut rexpr: Vec <rExpr > = collect_not_nested_let_tokens (&func_body);
    let fn_lines = get_lines_in_fn (&mut func_body);
    let all_assigns = collect_all_assigns (&func_body);
    let empty_rexpr = rExpr::new();
    let all_assigns = if let Some (x) = all_assigns { x } else { vec! [empty_rexpr] };
    let rexpr_for_loop: Option < rExpr > = token_for_loop (&func_body, 0);
    dbg! (&rexpr);
    func_body = rexpr.clone().into_iter().map(|x| {x.txt}).collect();
    rexpr.extend ( fn_lines );
    rexpr.extend (all_assigns);
  //  if let Some (x) = rexpr_for_loop { rexpr.push (x); }
    let mut var_list = String::new ();
    let mut ln = String::new ();
    for got in rexpr {
        leave_file_mark ("/tmp/got", &got.txt.clone() );
        ln = format! ("\n{}\n", got.txt.clone());
        var_list.push_str(ln.as_str() );
    }
    //let vars: String = rexpr.into_iter().map (|i| -> String { format! ("\n{}", i.txt) } ).collect();
    let new_end = quote! {
        println! ("var list {}\nnew end was successfully added", #var_list);
    }.to_string();
    leave_file_mark ("/tmp/ending", &new_end);
    let modified_func_body =  edit::rewrite_last_exit (&func_body, &new_end);
    dbg! (&modified_func_body);
    let out: TokenStream2 = modified_func_body.parse().unwrap();
    return out.into()
}
   #[inline]
   fn prnt_func (name_fn: &str, input: TokenStream) -> (TokenStream2, syn::Ident) {
        let func_body = input.to_string();
        let name_fn_str = name_fn.strn();
        let name_fn = strn_to_Ident (&name_fn);
        // let fn_name = parse_macro_input!( name_fn as LitStr ).value();
    
    // Convert the string to an Ident
    //let name_fn = syn::Ident::new(&name_fn, proc_macro2::Span::call_site());
        let out = quote! {
            pub fn #name_fn () {
            //println!("++++++++++++++++++++++++++++++");
                println! ("{}", #func_body);
            }
        };
       // let out: ItemFn = parse_quote!( out as ItemFn );
       let sub_fn: proc_macro2::TokenStream = out.into();
       let sub_fn_id = syn::Ident::new( &name_fn_str, proc_macro2::Span::call_site());
        return (sub_fn, sub_fn_id)
   }
fn strn_to_LitStr(s: &str) -> LitStr {
    LitStr::new(s, Span::call_site())
}
fn strn_to_Ident(s: &str) -> syn::Ident {
    syn::Ident::new(s, Span::call_site())
}
#[proc_macro_attribute]
pub fn prnt_vars0(_attr: TokenStream, item: TokenStream) -> TokenStream {
use syn::spanned::Spanned;
    let item_fn = item.clone();
    let mut input_fn = parse_macro_input!( item_fn as ItemFn);
     let span = input_fn.attrs.first().map(|attr| attr.span()).unwrap_or_else(proc_macro2::Span::call_site);
    let fn_name = &input_fn.sig.ident;
    let mut new_stmts = Vec::new();

    for stmt in input_fn.block.stmts {
        if let Stmt::Local(local) = &stmt {
           // let line_num = local.pat.span().start().line;
            //let mut var_names = Vec::new();
            
            // Recursively collect all identifiers from the pattern
            //collect_idents(&local.pat, &mut var_names);
            let span = local.init.as_ref().unwrap().expr.clone(); // works
          //  let span = local.pat.attrs.clone();
            let local_pat = format! ("{:?}\nspanЪЪ {:?}", &local, span );
            
            //if !var_names.is_empty() {
                let print_stmts = /*var_names.iter().map(|ident| { */
                    quote! {
                       /* println!("[VAR] {} = {:?} (declared at line {})", 
                            stringify!(#ident), #ident, #line_num); */
                        println! ("Loc {:?}", #local_pat);
                    };
            //}
                
                new_stmts.push(quote! {
                    #stmt
                    #print_stmts
                });
                //continue;
            }
        new_stmts.push(quote! { #stmt });
        }

    let expanded = quote! {
        fn #fn_name() {
            println!("[FN] Entering {}", stringify!(#fn_name));
            #(#new_stmts)*
        }
    };

    expanded.into()
}

// Helper function to recursively collect identifiers from patterns
/* fn collect_idents(pat: &Pat, idents: &mut Vec<syn::Ident>) {
    match pat {
        Pat::Ident(pat_ident) => {
            idents.push(pat_ident.ident.clone());
        }
        Pat::Tuple(tuple) => {
            for elem in &tuple.elems {
                collect_idents(elem, idents);
            }
        }
        Pat::Struct(struct_pat) => {
            for field in &struct_pat.fields {
                collect_idents(&field.pat, idents);
            }
        }
        Pat::TupleStruct(tuple_struct) => {
            for elem in tuple_struct.path.segments.iter() {
                collect_idents(elem, idents);
            }
        }
        Pat::Slice(slice) => {
            for elem in &slice.elems {
                collect_idents(elem, idents);
            }
        }
          Pat::Box(box_pat) => {
            collect_idents(&box_pat.pat, idents);
        }
        Pat::Ref(ref_pat) => {
            collect_idents(&ref_pat.pat, idents);
        }
        Pat::Range(range) => {
            if let Some(start) = &range.start {
                collect_idents(start, idents);
            }
            if let Some(end) = &range.end {
                collect_idents(end, idents);
            }
        }
        Pat::Wild(_) => {}
        _ => {} // Ignore other pattern types
    }
} */
//pub use crate::inject_after_hello; 
// https://www.freecodecamp.org/news/procedural-macros-in-rust/
