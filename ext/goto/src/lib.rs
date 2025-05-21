//mod goto;
//pub use crate::goto::{label, goto};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, Stmt};
use proc_macro2::{TokenStream as TokenStream2, Span};
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
pub fn inject(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input: ItemFn = parse_macro_input!(item as ItemFn);
    let mut new_stmts = Vec::<Stmt>::new(); 

    for stmt in input.block.stmts {
        // Push the original statement
        new_stmts.push(stmt.clone());
    }
    let add_to: Stmt = syn::parse_quote! {
                println!("yst {:?}", input.block.stmts);
            };
            new_stmts.push ( add_to );
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
//pub use crate::inject_after_hello; 
// https://www.freecodecamp.org/news/procedural-macros-in-rust/
