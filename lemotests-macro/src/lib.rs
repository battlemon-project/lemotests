#![feature(let_else)]
extern crate core;

mod blueprint;
mod errors;
mod handler;
mod schema;

use errors::*;
use handler::*;
use proc_macro::TokenStream;
use schema::*;

/// The macro used to generate the helper methods for the `State<T>` struct from `lemotests` crate.
///
/// It deserializes JSON Scheme from the path or paths in arguments
/// and generate the helper methods for the `State<T>` struct for predefined accounts (Alice, Bob, Charlie, etc.).
///
/// # Example
///
/// `contract_scheme.json`:
/// ```json
///{
///  "name": "contract_name",
///  "functions": [
///     {
///       "name": "function_name",
///       "initable": false,
///       "kind": "call",
///       "arguments": [
///         {
///           "name": "argument_one",
///           "type": "String"
///         },
///        {
///          "name": "argument_two",
///          "type": "u64"
///        }
///       ]
///     }
///  ]
///}
/// ```
///
/// ```no_run
/// use lemotests::prelude::*;
/// use lemotests_macro::add_helpers;
///
/// add_helpers!("contract_scheme.json");
/// // you can also use add_helpers!("contract_scheme.json", "contract_scheme2.json");
/// ```
///
/// It generates methods like this:
///```no_run
/// use lemotests::prelude::*;
///
/// let bchain = StateBuilder::testnet()
///                    .with_contract("contract_name", "path/to/contract",Near(10))?
///                    .with_alice(Near(10))?
///                    .build()
///                    .await?;
///
/// bchain.call_contract_name_function_name("argument_one", 777)?;
/// // or
/// bchain.alice_call_contract_name_function_name("argument_one", 777)?;
/// ```
#[proc_macro]
pub fn add_helpers(item: TokenStream) -> TokenStream {
    handle_input_tt(item).unwrap().into()
}
