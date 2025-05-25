//mod goto;
//pub use crate::goto::{label, goto};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, ItemFn, LitStr, Stmt, Meta, MetaList, MetaNameValue, punctuated::Punctuated, Attribute,
token::Comma, Expr};
use proc_macro2::{TokenStream as TokenStream2, Span};
mod lex;
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
#[proc_macro_attribute]
pub fn inject_tst(_attrs: TokenStream, item: TokenStream) -> TokenStream {
    let item_str = format! ("{}", item.clone() );
    let mut input: ItemFn = parse_macro_input!(item as ItemFn);
    let attr_cpy = _attrs.clone();
    let attr_in: ItemFn = parse_macro_input!( attr_cpy as ItemFn);
    let attrss = &attr_in.attrs;
    println!("tssst {} val {}", _attrs.to_string(), "42");
  for attr in attrss {
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
}

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
    return TokenStream::from(quote! { #input })
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
//pub use crate::inject_after_hello; 
// https://www.freecodecamp.org/news/procedural-macros-in-rust/
