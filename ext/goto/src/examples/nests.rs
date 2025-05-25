/*

Skip to content
Search Gists
All gists
Back to GitHub
Sign in
Sign up

Instantly share code, notes, and snippets.
@kepler-5
kepler-5/comp_multi.rs
Last active May 16, 2025 03:39

Code
Revisions 2
Stars 25
Forks 3
Clone this repository at &lt;script src=&quot;https://gist.github.com/kepler-5/065346184523890310e38bcf9adff03e.js&quot;&gt;&lt;/script&gt;
Python comprehension proc macro that handles multiple nested for-if-clauses, flattening nested structure
comp_multi.rs
// This is free and unencumbered software released into the public domain.

// Anyone is free to copy, modify, publish, use, compile, sell, or
// distribute this software, either in source code form or as a compiled
// binary, for any purpose, commercial or non-commercial, and by any
// means.

// In jurisdictions that recognize copyright laws, the author or authors
// of this software dedicate any and all copyright interest in the
// software to the public domain. We make this dedication for the benefit
// of the public at large and to the detriment of our heirs and
// successors. We intend this dedication to be an overt act of
// relinquishment in perpetuity of all present and future rights to this
// software under copyright law.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
// IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.

// For more information, please refer to <https://unlicense.org/>




// Python comprehension proc macro that handles multiple nested for-if-clauses, flattening nested structure.
// Example:
//
//     let vec_of_vecs = vec![vec![1, 2, 3], vec![4, 5, 6]];
//
//     let result = comp![x for vec in vec_of_vecs for x in vec].collect::<Vec<_>>();
//     assert_eq!(result, [1, 2, 3, 4, 5, 6]);
//

use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Expr, Pat, Token,
};

struct Comprehension {
    mapping: Mapping,
    for_if_clause: ForIfClause,
    additional_for_if_clauses: Vec<ForIfClause>,
}

impl Parse for Comprehension {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            mapping: input.parse()?,
            for_if_clause: input.parse()?,
            additional_for_if_clauses: parse_zero_or_more(input),
        })
    }
}

impl ToTokens for Comprehension {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let all_for_if_clauses =
            std::iter::once(&self.for_if_clause).chain(&self.additional_for_if_clauses);
        let mut innermost_to_outermost = all_for_if_clauses.rev();

        let mut output = {
            // innermost is a special case--here we do the mapping
            let innermost = innermost_to_outermost
                .next()
                .expect("We know we have at least one ForIfClause (self.for_if_clause)");
            let ForIfClause {
                pattern,
                sequence,
                conditions,
            } = innermost;

            let Mapping(mapping) = &self.mapping;

            quote! {
                core::iter::IntoIterator::into_iter(#sequence).filter_map(move |#pattern| {
                    (true #(&& (#conditions))*).then(|| #mapping)
                })
            }
        };

        // Now we walk through the rest of the ForIfClauses, wrapping the current `output` in a new layer of iteration each time.
        // We also add an extra call to '.flatten()'.
        output = innermost_to_outermost.fold(output, |current_output, next_layer| {
            let ForIfClause {
                pattern,
                sequence,
                conditions,
            } = next_layer;
            quote! {
                core::iter::IntoIterator::into_iter(#sequence).filter_map(move |#pattern| {
                    (true #(&& (#conditions))*).then(|| #current_output)
                })
                .flatten()
            }
        });

        tokens.extend(output)
    }
}

struct Mapping(Expr);

impl Parse for Mapping {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse().map(Self)
    }
}

struct ForIfClause {
    pattern: Pat,
    sequence: Expr,
    conditions: Vec<Condition>,
}

impl Parse for ForIfClause {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        _ = input.parse::<Token![for]>()?;
        let pattern = Pat::parse_single(input)?;
        _ = input.parse::<Token![in]>()?;
        let sequence = input.parse()?;
        let conditions = parse_zero_or_more(input);
        Ok(Self {
            pattern,
            sequence,
            conditions,
        })
    }
}

fn parse_zero_or_more<T: Parse>(input: ParseStream) -> Vec<T> {
    let mut result = Vec::new();
    while let Ok(item) = input.parse() {
        result.push(item);
    }
    result
}

struct Condition(Expr);

impl Parse for Condition {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        _ = input.parse::<Token![if]>()?;
        input.parse().map(Self)
    }
}

impl ToTokens for Condition {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.0.to_tokens(tokens)
    }
}

#[proc_macro]
pub fn comp(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let c = parse_macro_input!(input as Comprehension);
    quote! { #c }.into()
}

*/
// https://gist.github.com/kepler-5/065346184523890310e38bcf9adff03e