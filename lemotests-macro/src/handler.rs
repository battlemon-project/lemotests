use crate::{ContractSchema, FunctionSchema, MacrosError};
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{ExprLit, Lit, Type};

pub(crate) fn handle_input_tt(input: proc_macro::TokenStream) -> Result<TokenStream, MacrosError> {
    let tokens = match syn::parse::<ExprLit>(input) {
        Ok(ExprLit {
            lit: Lit::Str(path),
            ..
        }) => {
            let mut reader = crate::read_json_schema_from_file(path.value())?;
            let schema = crate::deserialize_json_schema(&mut reader)?;
            compose_helpers_tt(&schema)
        }
        _ => syn::Error::new(
            Span::call_site(),
            "parse can only be used with string literals",
        )
        .to_compile_error(),
    };

    Ok(tokens)
}

fn compose_helpers_tt(schema: &ContractSchema) -> TokenStream {
    let (declarations_tt, implementations_tt): (Vec<_>, Vec<_>) =
        compose_methods_tt("alice", schema).into_iter().unzip();

    quote! {
        pub trait Helper<T> {
           #(#declarations_tt ;)*
        }

        impl<T> Helper<T> for lemotests::State<T>
        where
            T: lemotests::workspaces::DevNetwork,
        {
            #(#implementations_tt)*
        }
    }
}

fn compose_methods_tt<'a>(
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
            let trait_method_name_ident = format_ident!("{account}_{kind}_{contract}_{contract_function}");

            let arguments_tt = compose_arguments_tt(function_schema);
            let flat_arguments_tt: Vec<_> = arguments_tt
                .iter()
                .map(|(name, r#type)| quote! { #name: #r#type })
                .collect();

            let declaration_tt = quote! {
               fn #trait_method_name_ident(self, #(#flat_arguments_tt),*) -> Result<lemotests::TxWrapper<T>, lemotests::HelperError>
            };

            let args_without_types_tt = arguments_tt
                .iter()
                .map(|(arg, _)| quote!(#arg));

            let implementation_tt = quote! {
                fn #trait_method_name_ident(self, #(#flat_arguments_tt),*) -> Result<lemotests::TxWrapper<T>, lemotests::HelperError> {
                    let account = self.account_key(#account).cloned();
                    let contract = self.contract_key(#contract).cloned();

                    if account.is_none() && contract.is_none() {
                        return Err(lemotests::HelperError::AccountAndContractNotFound("#account, #contract".to_owned()));
                    };

                    let mut json_args = serde_json::Map::new();
                    #(
                        let value = serde_json::to_value(#args_without_types_tt).expect("Fail to serialize argument to `Value`");
                        json_args.insert(stringify!(#args_without_types_tt).into(), value);
                    )*

                    let tx = lemotests::TxWrapper::new(account, contract, #contract_function.to_owned(), json_args, self);

                    Ok(tx)
                }
            };

            (declaration_tt, implementation_tt)
        })
        .collect()
}

fn compose_arguments_tt(function_schema: &FunctionSchema) -> Vec<(TokenStream, TokenStream)> {
    function_schema
        .arguments
        .iter()
        .map(|argument_schema| {
            let name = &argument_schema.name;
            let type_ = &argument_schema.r#type.replacen("String", "&str", 1);
            let name_ident = format_ident!("{name}");
            let type_ident: Type = syn::parse_str(type_).unwrap();

            (name_ident.into_token_stream(), type_ident.to_token_stream())
        })
        .collect()
}
