// derive/src/datakind.rs
// Copyright (C) 2018 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================

// Stdlib imports

use proc_macro::TokenStream;

// Third-party imports
use syn::{DataEnum, DeriveInput, Generics, Ident, Variant};

// Local imports

// ===========================================================================
//
// ===========================================================================

#[proc_macro_derive(IsDataKind)]
pub fn is_datakind(input: TokenStream) -> TokenStream
{
    // Parse input tokens into a syntax tree
    let input: DeriveInput = syn::parse(input).unwrap();

    // Build the impl
    let expanded = impl_is_datakind(&input);

    // Hand output tokens back to compiler
    expanded.into()
}

fn mk_code_impl(
    enum_name: &Ident, enum_generics: &Generics, cases: &Vec<quote::Tokens>
) -> quote::Tokens
{
    if enum_generics.params.len() > 0 {
        quote! {
            impl #enum_name#enum_generics {
                #(#cases)*
            }
        }
    } else {
        quote! {
            impl #enum_name {
                #(#cases)*
            }
        }
    }
}

fn process_enum_variants(
    enum_name: &Ident, enum_generics: &Generics, enumdef: &DataEnum
) -> quote::Tokens
{
    let cases: Vec<_> = enumdef
        .variants
        .iter()
        .map(|var| {
            // Panic if the variant is a unit
            match var.fields {
                syn::Fields::Unit | syn::Fields::Named(_) => {
                    panic!(
                        "#[derive(IsDataKind)] currently does not support \
                         unit or struct variants"
                    );
                }
                _ => {}
            }

            let var_name = &var.ident;
            let fn_name = {
                let new_name = var_name.display().to_lowercase();
                Ident::from(new_name)
            };
            quote! {
                fn is_#var_name(&self) -> bool {
                    match *self {
                        #num_name::#var_name(_) => true,
                        _ => false,
                    }
                }
            }
        })
        .collect();

    mk_code_impl(enum_name, enum_generics, &cases)
}

fn impl_code_convert(ast: &syn::DeriveInput) -> quote::Tokens
{
    let enum_name = &ast.ident;
    if let syn::Data::Enum(ref enumdef) = ast.data {
        process_num_variants(enum_name, enumdef)
    } else {
        panic!("#[derive(IsDataKind)] is only defined for enums not structs");
    }
}

// ===========================================================================
//
// ===========================================================================
