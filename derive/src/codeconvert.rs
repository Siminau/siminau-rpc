// src/codeconvert.rs
// Copyright (C) 2018 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================

// Stdlib imports

// Third-party imports
use num::ToPrimitive;
use quote;
use syn::{Data, DataEnum, DeriveInput, Expr, Fields, Ident, Lit};

// Local imports

// ===========================================================================
//
// ===========================================================================

struct Literal<'a>
{
    num: &'a Lit,
}

impl<'a> From<&'a Lit> for Literal<'a>
{
    fn from(num: &'a Lit) -> Self
    {
        Self { num: num }
    }
}

impl<'a> ToPrimitive for Literal<'a>
{
    fn to_i64(&self) -> Option<i64>
    {
        match self.num {
            &Lit::Int(ref int) => Some(int.value() as i64),
            _ => None,
        }
    }

    fn to_u64(&self) -> Option<u64>
    {
        match self.num {
            &Lit::Int(ref int) => Some(int.value()),
            _ => None,
        }
    }
}

fn mk_code_impl(
    name: &Ident, cases: &Vec<quote::Tokens>, int_type: Ident, maxnum: u64
) -> quote::Tokens
{
    quote! {
        impl CodeConvert<#name> for #name {
            type int_type = #int_type;

            fn from_number(num: #int_type) -> Result<#name, CodeValueError> {
                Self::from_u64(num as u64)
            }

            fn from_u64(num: u64) -> Result<#name, CodeValueError> {
                match num {
                    #(#cases),* ,
                    _ => Err(CodeValueError {code: num})
                }
            }

            fn to_number(&self) -> #int_type {
                self.clone() as #int_type
            }

            fn to_u64(&self) -> u64 {
                self.clone() as u64
            }

            fn max_number() -> u64 {
                #maxnum
            }

            fn cast_number(n: u64) -> Option<#int_type> {
                let maxval = #int_type::max_value() as u64;
                if n <= maxval {
                    Some(n as #int_type)
                } else {
                    None
                }
            }
        }
    }
}

fn process_enum_variants(enum_name: &Ident, enumdef: &DataEnum)
    -> quote::Tokens
{
    let mut num = 0;
    let mut maxnum: u64 = 0;

    let cases: Vec<_> = enumdef
        .variants
        .iter()
        .map(|var| {
            // Panic if the variant is a struct or a tuple
            match var.fields {
                Fields::Unnamed(_) | Fields::Named(_) => {
                    panic!(
                        "#[derive(CodeConvert)] currently does not support \
                         tuple or struct variants"
                    );
                }
                _ => {}
            }

            // Create variant identifier
            let variant = &var.ident;
            let ident = quote! { #enum_name::#variant };

            if let Some((_, ref d)) = var.discriminant {
                if let &Expr::Lit(ref l) = d {
                    let lit = Literal::from(&l.lit);
                    num = match lit.to_u64() {
                        None => panic!(
                            "#[derive(CodeConvert)] only supports mapping to \
                             u64"
                        ),
                        Some(v) => v,
                    };
                } else {
                    panic!("#[derive(CodeConvert)] only supports literals")
                }
            }

            if num > maxnum {
                maxnum = num;
            }
            let ret = quote! { #num => Ok(#ident) };
            num += 1;
            ret
        })
        .collect();

    let u32_max = u32::max_value() as u64;
    let u16_max = u16::max_value() as u64;
    let u8_max = u8::max_value() as u64;
    let int_type = if maxnum > u32_max {
        Ident::from("u64")
    } else if maxnum > u16_max {
        Ident::from("u32")
    } else if maxnum > u8_max {
        Ident::from("u16")
    } else {
        Ident::from("u8")
    };
    mk_code_impl(enum_name, &cases, int_type, maxnum)
}

pub fn impl_code_convert(ast: &DeriveInput) -> quote::Tokens
{
    let enum_name = &ast.ident;
    if let Data::Enum(ref enumdef) = ast.data {
        process_enum_variants(enum_name, enumdef)
    } else {
        panic!("#[derive(IsDataKind)] is only defined for enums not structs");
    }
}

// pub fn impl_code_convert1(ast: &syn::DeriveInput) -> quote::Tokens
// {
//     if let syn::Body::Enum(ref body) = ast.body {
//         let name = &ast.ident;
//         let mut num = 0;
//         let mut maxnum: u64 = 0;
//         let cases: Vec<_> = body.iter()
//             .map(|case| {
//                 // Panic if the variant is a struct or tuple
//                 if let syn::VariantData::Unit = case.data {
//                     // Create variant identifier
//                     let variant = &case.ident;
//                     let ident = quote! { #name::#variant };

//                     // If literal number assigned to variant, assign to num
//                     if let Some(ref d) = case.discriminant {
//                         if let &syn::ConstExpr::Lit(ref l) = d {
//                             let lit = Literal::from(l);
//                             num = match lit.to_u64() {
//                                 None => panic!(
//                                     "#[derive(CodeConvert)] only supports \
//                                      mapping to u64"
//                                 ),
//                                 Some(v) => v,
//                             };
//                         } else {
//                             panic!(
//                                 "#[derive(CodeConvert)] only supports literals"
//                             )
//                         }
//                     }
//                     if num > maxnum {
//                         maxnum = num;
//                     }
//                     let ret = quote! { #num => Ok(#ident) };
//                     num += 1;
//                     ret
//                 } else {
//                     panic!(
//                         "#[derive(CodeConvert)] currently does not support \
//                          tuple or struct variants"
//                     );
//                 }
//             })
//             .collect();

//         let u32_max = u32::max_value() as u64;
//         let u16_max = u16::max_value() as u64;
//         let u8_max = u8::max_value() as u64;
//         let int_type = if maxnum > u32_max {
//             syn::Ident::from("u64")
//         } else if maxnum > u16_max {
//             syn::Ident::from("u32")
//         } else if maxnum > u8_max {
//             syn::Ident::from("u16")
//         } else {
//             syn::Ident::from("u8")
//         };
//         mk_code_impl(name, &cases, int_type, maxnum)
//     } else {
//         panic!("#[derive(CodeConvert)] is only defined for enums not structs");
//     }
// }

// ===========================================================================
//
// ===========================================================================
