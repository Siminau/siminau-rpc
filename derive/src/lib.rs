// src/lib.rs
// Copyright (C) 2017 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Externs
// ===========================================================================
#![recursion_limit = "1024"]

// Stdlib externs
extern crate proc_macro;

// Third-party externs
extern crate num;

#[macro_use]
extern crate quote;

extern crate syn;

// Local externs

// ===========================================================================
// Modules
// ===========================================================================

mod codeconvert;
mod datakind;
mod from_message;
mod from_version_message;

// ===========================================================================
// Imports
// ===========================================================================

// Stdlib imports
use proc_macro::TokenStream;

// Third-party imports
use syn::DeriveInput;

// Local imports
use codeconvert::impl_code_convert;
use datakind::impl_is_datakind;
use from_message::impl_from_message;
use from_version_message::impl_from_version_message;

// ===========================================================================
// Helper
// ===========================================================================

fn build_derive<F>(input: TokenStream, expand: F) -> TokenStream
where
    F: Fn(&DeriveInput) -> quote::Tokens,
{
    // Parse input tokens into a syntax tree
    let input: DeriveInput = syn::parse(input).unwrap();

    // Build the impl
    let expanded = expand(&input);

    // Hand output tokens back to compiler
    expanded.into()
}

// ===========================================================================
// IsDataKind
// ===========================================================================

#[proc_macro_derive(IsDataKind)]
pub fn is_datakind(input: TokenStream) -> TokenStream
{
    build_derive(input, impl_is_datakind)
}

// ===========================================================================
// CodeConvert
// ===========================================================================

#[proc_macro_derive(CodeConvert)]
pub fn code_convert(input: TokenStream) -> TokenStream
{
    build_derive(input, impl_code_convert)
}

// ===========================================================================
// FromMessage
// ===========================================================================

#[proc_macro_derive(FromMessage)]
pub fn from_message(input: TokenStream) -> TokenStream
{
    build_derive(input, impl_from_message)
}

// ===========================================================================
// FromVersionMessage
// ===========================================================================

// This depends on simianu_rpc::message being available
#[proc_macro_derive(FromVersionMessage)]
pub fn from_version_message(input: TokenStream) -> TokenStream
{
    build_derive(input, impl_from_version_message)
}

// ===========================================================================
//
// ===========================================================================
