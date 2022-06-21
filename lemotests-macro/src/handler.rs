use crate::{ContractSchema, FunctionSchema, MacrosError};
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{ExprLit, Lit};

pub(crate) fn handle_input(input: proc_macro::TokenStream) -> Result<TokenStream, MacrosError> {
    let tokens = match syn::parse::<ExprLit>(input) {
        Ok(ExprLit {
            lit: Lit::Str(path),
            ..
        }) => {
            let mut reader = crate::read_json_schema_from_file(path.value())?;
            let schema = crate::deserialize_json_schema(&mut reader)?;
            compose_helpers(&schema)
        }
        _ => syn::Error::new(
            Span::call_site(),
            "parse can only be used with string literals",
        )
        .to_compile_error(),
    };

    Ok(tokens)
}

fn compose_helpers(schema: &ContractSchema) -> TokenStream {
    let (declarations, implementations): (Vec<_>, Vec<_>) =
        compose_methods("alice", schema).into_iter().unzip();

    quote! {
        pub trait Helper {
           #(#declarations ;)*
        }

        impl<T> Helper for lemotests::State<T>
        where
            T: lemotests::workspaces::DevNetwork,
        {
            #(#implementations)*
        }
    }
}

fn compose_methods<'a>(
    account: &'a str,
    schema: &'a ContractSchema,
) -> Vec<(TokenStream, TokenStream)> {
    schema
        .functions
        .iter()
        .map(move |function_schema| {
            let contract = &schema.name;
            let kind = &function_schema.kind;
            let contract_function = &function_schema.name;
            let trait_method_name = format_ident!("{account}_{kind}_{contract}_{contract_function}");

            let arguments = compose_arguments(function_schema);
            let flat_arguments: Vec<_> = arguments
                .iter()
                .map(|(name, r#type)| quote! { #name: #r#type })
                .collect();

            let declaration = quote! {
               fn #trait_method_name(&self, #(#flat_arguments),*) -> Result<lemotests::TxWrapper<'_>, lemotests::HelperError>
            };

            let args_without_types = arguments
                .iter()
                .map(|(arg, _)| quote!(#arg));

            let implementation = quote! {
                fn #trait_method_name(&self, #(#flat_arguments),*) -> Result<lemotests::TxWrapper<'_>, lemotests::HelperError> {
                    let mut json_args = serde_json::Map::new();

                    #(json_args.insert(stringify!(#args_without_types).into(), #args_without_types.into());)*
                    let account = self.account(#account).ok();
                    let contract = self.contract(#contract).ok();
                    if account.is_none() && contract.is_none() {
                        return Err(lemotests::HelperError::AccountAndContractNotFound(format!("{}, {}", #account, #contract)));
                    };
                    let tx = lemotests::TxWrapper::new(account, contract, #contract_function, json_args);

                    Ok(tx)
                }
            };

            (declaration, implementation)
        })
        .collect()
}

fn compose_arguments(function_schema: &FunctionSchema) -> Vec<(TokenStream, TokenStream)> {
    function_schema
        .arguments
        .iter()
        .map(|argument_schema| {
            let name = &argument_schema.name;
            let r#type = &argument_schema.r#type;
            let name_ident = format_ident!("{name}");
            let argument_name = quote! { #name_ident };
            let argument_type = if r#type == "String" {
                quote! { &str }
            } else {
                let type_ident = format_ident!("{type}");
                quote! { #type_ident }
            };

            (argument_name, argument_type)
        })
        .collect()
}
