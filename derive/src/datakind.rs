// src/datakind.rs
// Copyright (C) 2018 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================

// Stdlib imports

// Third-party imports
use quote;
use syn::{Data, DataEnum, DeriveInput, Fields, Generics, Ident};

// Local imports

// ===========================================================================
//
// ===========================================================================

fn mk_code_impl(
    enum_name: &Ident, enum_generics: &Generics, cases: &Vec<quote::Tokens>
) -> quote::Tokens
{
    if enum_generics.params.len() > 0 {
        quote! {
            impl#enum_generics #enum_name#enum_generics {
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
                Fields::Unit | Fields::Named(_) => {
                    panic!(
                        "#[derive(IsDataKind)] currently does not support \
                         unit or struct variants"
                    );
                }
                _ => {}
            }

            let var_name = &var.ident;
            let fn_name = {
                let new_name = format!("{}", var_name).to_lowercase();
                Ident::from(format!("is_{}", new_name))
            };
            quote! {
                pub fn #fn_name(&self) -> bool {
                    match *self {
                        #enum_name::#var_name(_) => true,
                        _ => false,
                    }
                }
            }
        })
        .collect();

    mk_code_impl(enum_name, enum_generics, &cases)
}

pub fn impl_is_datakind(ast: &DeriveInput) -> quote::Tokens
{
    let enum_name = &ast.ident;
    let enum_generics = &ast.generics;
    if let Data::Enum(ref enumdef) = ast.data {
        process_enum_variants(enum_name, enum_generics, enumdef)
    } else {
        panic!("#[derive(IsDataKind)] is only defined for enums not structs");
    }
}

// ===========================================================================
//
// ===========================================================================
