/*!
An implementation detail of abi_stable.
*/

#![recursion_limit="192"]
// #![deny(unused_variables)]
// #![deny(unused_imports)]
// #![deny(unused_parens)]
// #![deny(unused_assignments)]
// #![deny(unused_mut)]
#![deny(unreachable_patterns)]
#![deny(unused_doc_comments)]
#![deny(unconditional_recursion)]

extern crate proc_macro;


mod parse_utils;
mod structural_alias_impl;
mod structural_derive;
mod str_or_ident;
mod tokenizers;


use proc_macro::TokenStream as TokenStream1;
use proc_macro2::{
    TokenStream as TokenStream2,
};

use quote::quote;



/**


This macro is documented in structural::docs::structural_macro

*/

#[proc_macro_derive(Structural, attributes(struc))]
pub fn derive_structural(input: TokenStream1) -> TokenStream1 {
    parse_or_compile_err( input, structural_derive::derive ).into()
}


#[proc_macro]
pub fn structural_alias_impl(input: TokenStream1) -> TokenStream1 {
    parse_or_compile_err(input,structural_alias_impl::macro_impl).into()
}

/**
The implementation of the ti macro when const parameters aren't supported.
*/
#[proc_macro]
pub fn tstr_impl(input: TokenStream1) -> TokenStream1 {
    use crate::{
        parse_utils::ParsePunctuated,
        tokenizers::tstring_tokenizer,
        str_or_ident::StrOrIdent,
    };


    parse_or_compile_err(input,|strings_: ParsePunctuated<StrOrIdent,syn::Token!(,)>|{
        let strings=strings_.list;
        let tokens=if strings.len()==1 {
            let string=strings[0].value();
            let tstring=tstring_tokenizer(string);

            quote!(
                use structural::proc_macro_reexports::*;

                pub const VALUE:#tstring=MarkerType::MTVAL;
            )
        }else{
            let mut prev_strings=Vec::<String>::new();
            let mut tstring=Vec::new();
            for string_lit in strings {
                let string=string_lit.value();
                if prev_strings.contains(&string) {
                    return Err(syn::Error::new(
                        string_lit.span(),
                        "Field names cannot be used more than once"
                    ));
                }else{
                    prev_strings.push(string.clone());
                    tstring.push(tstring_tokenizer(string));
                }
            }
            quote!(
                use structural::proc_macro_reexports::*;

                pub const VALUE:TStringSet<(#(#tstring),*)>=unsafe{
                    TStringSet::new()
                };
            )
        };

        Ok(tokens)
    }).into()
}





////////////////////////////////////////////////////////////////////////////////


fn parse_or_compile_err<P,F>(input:TokenStream1,f:F)->TokenStream2
where 
    P:syn::parse::Parse,
    F:FnOnce(P)->Result<TokenStream2,syn::Error>
{
    syn::parse::<P>(input)
        .and_then(f)
        .unwrap_or_else(|e| e.to_compile_error() )
}
