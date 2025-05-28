//mod goto;
//pub use crate::goto::{label, goto};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, ItemFn, LitStr, Stmt, Meta, MetaList, MetaNameValue, punctuated::Punctuated, Attribute,
token::Comma, Expr, Lit};
use proc_macro2::{TokenStream as TokenStream2, Span};
mod lex;
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
   let sub_fn = prnt_func ("lets_prnt_func", item.clone());
   return sub_fn.clone();
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
    let mut args_meta: Meta = Meta::NameValue(meta_name_value);
    let mut stop_err = false;
    match syn::parse::<Meta>(args) {
        Ok(meta) => { args_meta = meta; },
        Err(e) => { let err = format! ("customized compiler's error {:#?}", e); stop_err = true;
                    return TokenStream::from (quote! {
                        pub fn lets_prnt_func () {
                            println! ("{}", #err);
                        }
                    })},
    };
    //if stop_err {compile_error! ("stop_err"); }
    match args_meta {
        Meta::Path(path) => { ext_quote.extend (
            quote! { println!("Simple attribute: {:?}", #path); }
            );
        }
        Meta::List(list) => { 
            let list = list.tokens;
        ext_quote.extend (
            quote! {println!("Attribute with args: {:?}", #list); }
            );
        }
        Meta::NameValue(name_value) => { 
            let ident = name_value.path.get_ident().unwrap();
            let value = name_value.value;
        ext_quote.extend (
           quote! { println!("Name-value attribute: {} = {:?}", 
                   #ident,
                   #value);
            } );
        }
    };
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
    return TokenStream::from(quote! { 
        pub fn lets_prnt_func () {
            println! ("input {} ext {}", #input_str, #ext_quote_str );
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
   #[inline]
   fn prnt_func (name_fn: &str, input: TokenStream) -> TokenStream {
        let func_body = input.to_string();
        let name_fn = strn_to_Ident (&name_fn);
        // let fn_name = parse_macro_input!( name_fn as LitStr ).value();
    
    // Convert the string to an Ident
    //let name_fn = syn::Ident::new(&name_fn, proc_macro2::Span::call_site());
        let out = quote! {
            pub fn #name_fn () {
                println! ("{}", #func_body);
            }
        };
       // let out: ItemFn = parse_quote!( out as ItemFn );
        return out.into()
   }
fn strn_to_LitStr(s: &str) -> LitStr {
    LitStr::new(s, Span::call_site())
}
fn strn_to_Ident(s: &str) -> syn::Ident {
    syn::Ident::new(s, Span::call_site())
}
//pub use crate::inject_after_hello; 
// https://www.freecodecamp.org/news/procedural-macros-in-rust/
