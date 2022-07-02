#![feature(let_else)]
extern crate core;

mod errors;
mod handler;
mod schema;
mod blueprint;

use errors::*;
use handler::*;
use proc_macro::TokenStream;
use schema::*;

#[proc_macro]
pub fn add_helpers(item: TokenStream) -> TokenStream {
    handle_input_tt(item).unwrap().into()
}
