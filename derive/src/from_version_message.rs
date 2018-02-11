// src/from_version_message.rs
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
                        "#[derive(FromVersionMessage)] currently does not support \
                         unit or struct variants"
                    );
                }
                Fields::Unnamed(ref f) => {
                    if f.unnamed.len() != 1 {
                        panic!(
                            "#[derive(FromVersionMessage)] does not support empty \
                             tuple variants or tuple variants with more than \
                             1 field"
                        );
                    }

                    &f.unnamed[0]
                }
            };

            let field_type = &field.ty;
            quote! {
                impl From<#field_type> for message::RequestMessage
                {
                    fn from(m: #field_type) -> message::RequestMessage
                    {
                        let vmsg = #enum_name::from(m);
                        message::RequestMessage::from(vmsg)
                    }
                }
            }
        })
        .collect();

    quote! {
        #(#cases)*
    }
}

pub fn impl_from_version_message(ast: &DeriveInput) -> quote::Tokens
{
    let enum_name = &ast.ident;
    if let Data::Enum(ref enumdef) = ast.data {
        process_enum_variants(enum_name, enumdef)
    } else {
        panic!(
            "#[derive(FromVersionMessage)] is only defined for enums not \
             structs"
        );
    }
}

// ===========================================================================
//
// ===========================================================================
