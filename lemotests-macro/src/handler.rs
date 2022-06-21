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

        impl<T: test_helpers::workspaces::DevNetwork> Helper for test_helpers::State<T> {
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

            let arguments_with_generics = compose_arguments(function_schema, true);
            let arguments_without_generics = compose_arguments(function_schema, false);
            let flat_arguments: Vec<_> = arguments_with_generics
                .iter()
                .map(|(name, r#type)| quote! { #name: #r#type })
                .collect();

            let declaration = quote! {
               fn #trait_method_name(&self, #(#flat_arguments),*) -> Result<(), test_helpers::HelperError>
            };

            let args_without_types = arguments_without_generics
                .iter()
                .map(|(arg, _)| quote!(#arg));

            let implementation = quote! {
                fn #trait_method_name(&self, #(#flat_arguments),*) -> Result<(), test_helpers::HelperError> {
                    let mut json_args = serde_json::Map::new();

                    #(json_args.insert(stringify!(#args_without_types).into(), #args_without_types.into());)*
                    let account = self.account(#account).ok();
                    let contract = self.contract(#contract).ok();
                    if account.is_none() && contract.is_none() {
                        return Err(test_helpers::HelperError::AccountAndContractNotFound(format!("{}, {}", #account, #contract)));
                    };
                    let tx = test_helpers::TxWrapper::new(account, contract, #contract_function, json_args);
                    // .call(self.worker(), #contract, #name);

                    Ok(())
                }
            };

            (declaration, implementation)
        })
        .collect()
}

fn compose_arguments(
    function_schema: &FunctionSchema,
    generic: bool,
) -> Vec<(TokenStream, TokenStream)> {
    function_schema
        .arguments
        .iter()
        .map(|argument_schema| {
            let name = &argument_schema.name;
            let r#type = &argument_schema.r#type;
            let name_ident = format_ident!("{name}");
            let argument_name = quote! { #name_ident };
            let argument_type = if r#type == "String" && generic {
                // quote! { impl AsRef<str> + serde::Serialize + serde::Deserialize}
                quote! { &str }
            } else {
                let type_ident = format_ident!("{type}");
                quote! { #type_ident }
            };

            (argument_name, argument_type)
        })
        .collect()
}
