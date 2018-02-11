// src/as_message.rs
// Copyright (C) 2018 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================

// Stdlib imports

// Third-party imports
use quote;
use syn::{Data, DataEnum, DeriveInput, Fields, Ident};

// Local imports

// ===========================================================================
//
// ===========================================================================

fn process_enum_variants(enum_name: &Ident, enumdef: &DataEnum)
    -> quote::Tokens
{
    let cases: Vec<_> = enumdef
        .variants
        .iter()
        .map(|var| {
            // Panic if the variant is a unit
            let field = match var.fields {
                Fields::Unit | Fields::Named(_) => {
                    panic!(
                        "#[derive(AsMessage)] currently does not support unit \
                         or struct variants"
                    );
                }
                Fields::Unnamed(ref f) => {
                    if f.unnamed.len() != 1 {
                        panic!(
                            "#[derive(AsMessage)] does not support empty \
                             tuple variants or tuple variants with more than \
                             1 field"
                        );
                    }

                    &f.unnamed[0]
                }
            };

            let var_name = &var.ident;
            let fn_name = {
                let new_name = format!("{}", var_name).to_lowercase();
                Ident::from(format!("as_{}", new_name))
            };
            let field_type = &field.ty;
            quote! {
                pub fn #fn_name(&self) -> Option<&#field_type>
                {
                    match *self {
                        #enum_name::#var_name(ref msg) => Some(msg),
                        _ => None
                    }
                }
            }
        })
        .collect();

    quote! {
        impl #enum_name {
            #(#cases)*
        }
    }
}

pub fn impl_as_message(ast: &DeriveInput) -> quote::Tokens
{
    let enum_name = &ast.ident;
    if let Data::Enum(ref enumdef) = ast.data {
        process_enum_variants(enum_name, enumdef)
    } else {
        panic!("#[derive(AsMessage)] is only defined for enums not structs");
    }
}

// ===========================================================================
//
// ===========================================================================
