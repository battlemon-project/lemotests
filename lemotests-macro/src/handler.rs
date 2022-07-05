use crate::blueprint::FunctionBlueprint;
use crate::{ContractSchema, MacrosError};
use lemotests::consts::ACCOUNTS;
use lemotests::TxKind;
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use std::path::Path;
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::{ExprLit, Lit, Token};

pub(crate) fn handle_input_tt(input: proc_macro::TokenStream) -> Result<TokenStream, MacrosError> {
    let parser = Punctuated::<ExprLit, Token![,]>::parse_terminated;
    let parsed = parser.parse(input);
    let tokens = match parsed {
        Ok(punctuated) => {
            let paths = get_paths(punctuated)?;
            let mut readers = crate::read_json_schemas_from_file(&paths)?;
            let schemas = crate::deserialize_json_schemas(&mut readers)?;
            compose_helper_trait_tt(&schemas)
        }
        _ => syn::Error::new(
            Span::call_site(),
            "parse can only be used with list of literal paths",
        )
        .to_compile_error(),
    };

    Ok(tokens)
}

fn get_paths<P>(punctuated: Punctuated<ExprLit, P>) -> Result<Vec<String>, MacrosError> {
    let mut ret = Vec::new();
    for expr_lit in punctuated.iter() {
        match expr_lit {
            ExprLit {
                lit: Lit::Str(lit), ..
            } => {
                let path = lit.value();
                ret.push(path);
            }
            _ => {
                return Err(MacrosError::DestructuringPunctuatedError(
                    "Must contain only `ExprLit` types".to_string(),
                ))
            }
        }
    }

    Ok(ret)
}

fn compose_helper_trait_tt(schemas: &[ContractSchema]) -> TokenStream {
    let mut all_declarations_tt = TokenStream::new();
    let mut all_implementations_tt = TokenStream::new();

    for schema in schemas {
        let (declarations_tt, implementations_tt) =
            compose_methods_for_accounts_tt(schema, &ACCOUNTS);
        declarations_tt.to_tokens(&mut all_declarations_tt);
        implementations_tt.to_tokens(&mut all_implementations_tt);
    }

    quote! {
        pub trait Helper<T>
        {
           #all_declarations_tt
        }

        impl<T> Helper<T> for lemotests::State<T>
        where
            T: lemotests::workspaces::DevNetwork,
        {
            #all_implementations_tt
        }
    }
}

fn compose_methods_for_accounts_tt<'a>(
    schema: &'a ContractSchema,
    accounts: &'a [&'a str],
) -> (TokenStream, TokenStream) {
    let mut declarations_tt = TokenStream::new();
    let mut implementations_tt = TokenStream::new();
    for blueprint in schema.blueprints(accounts) {
        let (temp_declarations_tt, temp_implementations_tt) =
            compose_method_for_accounts_tt(blueprint);

        temp_declarations_tt.to_tokens(&mut declarations_tt);
        temp_implementations_tt.to_tokens(&mut implementations_tt);
    }

    (declarations_tt, implementations_tt)
}

fn compose_method_for_accounts_tt(blueprint: FunctionBlueprint) -> (TokenStream, TokenStream) {
    let mut declarations_tt = TokenStream::new();
    let mut implementations_tt = TokenStream::new();
    let fn_args_tt = blueprint.args_tt();

    let method_name_ident = format_ident!("{}", blueprint.trait_method_name);
    let declaration_tt = quote! {
       fn #method_name_ident(self, #fn_args_tt) -> Result<lemotests::TxWrapper<T>, lemotests::HelperError>;
    };
    declaration_tt.to_tokens(&mut declarations_tt);

    let contract_function_name = &blueprint.contract_function_name;
    let account = blueprint.account.clone();
    let contract_name = &blueprint.contract_name;
    let args_without_types = blueprint.args_without_types();

    let implementation_tt = match blueprint.tx_kind() {
        TxKind::AccountCallContract => {
            let account = account.unwrap();
            quote! {
                fn #method_name_ident(self, #fn_args_tt) -> Result<lemotests::TxWrapper<T>, lemotests::HelperError> {
                    let account = self.account_key(#account).cloned();
                    let contract = self.contract_key(#contract_name).cloned();

                    if account.is_none() && contract.is_none() {
                        return Err(lemotests::HelperError::AccountAndContractNotFound(format!("{}, {}", #account, #contract_name)));
                    };

                    let mut json_args = lemotests::serde_json::Map::new();
                    #(
                        let value = lemotests::serde_json::to_value(#args_without_types).expect("Fail to serialize argument to `Value`");
                        json_args.insert(stringify!(#args_without_types).into(), value);
                    )*
                    let tx = lemotests::TxWrapper::new(account, contract, #contract_function_name.to_owned(), json_args, lemotests::TxKind::AccountCallContract, self);
                    Ok(tx)
                }
            }
        }
        TxKind::View => quote! {
            fn #method_name_ident(self, #fn_args_tt) -> Result<lemotests::TxWrapper<T>, lemotests::HelperError> {
                let contract = self.contract_key(#contract_name).cloned();

                if contract.is_none() {
                    return Err(lemotests::HelperError::ContractNotFound(format!("{}", #contract_name)));
                };

                let mut json_args = lemotests::serde_json::Map::new();
                #(
                    let value = lemotests::serde_json::to_value(#args_without_types).expect("Fail to serialize argument to `Value`");
                    json_args.insert(stringify!(#args_without_types).into(), value);
                )*
                let tx = lemotests::TxWrapper::new(None, contract, #contract_function_name.to_owned(), json_args, lemotests::TxKind::View, self);
                Ok(tx)
            }
        },
        TxKind::SelfContractCall => quote! {
            fn #method_name_ident(self, #fn_args_tt) -> Result<lemotests::TxWrapper<T>, lemotests::HelperError> {
                let contract = self.contract_key(#contract_name).cloned();

                if contract.is_none() {
                    return Err(lemotests::HelperError::ContractNotFound(format!("{}", #contract_name)));
                };

                let mut json_args = lemotests::serde_json::Map::new();
                #(
                    let value = lemotests::serde_json::to_value(#args_without_types).expect("Fail to serialize argument to `Value`");
                    json_args.insert(stringify!(#args_without_types).into(), value);
                )*
                let tx = lemotests::TxWrapper::new(None, contract, #contract_function_name.to_owned(), json_args, lemotests::TxKind::SelfContractCall, self);
                Ok(tx)
            }
        },
        TxKind::ViewAccount => unreachable!(),
    };

    implementation_tt.to_tokens(&mut implementations_tt);

    (declarations_tt, implementations_tt)
}
