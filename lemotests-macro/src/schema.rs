use crate::MacrosError;
use proc_macro2::TokenStream;
use quote::quote;
use serde::Deserialize;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use syn::{FnArg, Pat, PatIdent};

pub(crate) fn read_json_schema_from_file<P>(path: P) -> Result<BufReader<File>, MacrosError>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    Ok(BufReader::new(file))
}

pub(crate) fn deserialize_json_schema<R: Read>(
    reader: &mut BufReader<R>,
) -> Result<ContractSchema, MacrosError> {
    serde_json::from_reader(reader).map_err(MacrosError::DeserializeJsonSchemaError)
}

#[derive(Deserialize)]
pub(crate) struct ContractSchema {
    pub(crate) name: String,
    pub(crate) functions: Vec<FunctionSchema>,
}

impl ContractSchema {
    pub(crate) fn functions(&self) -> Vec<FunctionMetadata> {
        self.functions
            .iter()
            .map(|f| {
                let mut metadata = f.metadata();
                metadata.update_trait_method_name(|old_method_name| {
                    format!("{}_{}_{}", f.kind, self.name, old_method_name)
                });
                metadata
            })
            .collect()
    }
}

pub(crate) struct FunctionMetadata {
    pub(crate) contract_function_name: String,
    pub(crate) trait_method_name: String,
    pub(crate) args: Vec<FnArg>,
}

impl FunctionMetadata {
    pub(crate) fn new(contract_function_name: String, args: Vec<FnArg>) -> Self {
        Self {
            contract_function_name: contract_function_name.clone(),
            trait_method_name: contract_function_name,
            args,
        }
    }

    pub(crate) fn update_trait_method_name(&mut self, updater: impl Fn(&str) -> String) {
        self.trait_method_name = updater(&self.trait_method_name);
    }

    pub(crate) fn args_without_types(&self) -> Vec<&PatIdent> {
        self.args
            .iter()
            .map(|arg| {
                let FnArg::Typed(pat) = arg else { unreachable!() };
                let Pat::Ident(ident) = &*pat.pat else { unreachable!() };
                ident
            })
            .collect()
    }

    pub(crate) fn concat_with_accounts<'a, 'b: 'a>(
        &'a self,
        accounts: &'b [&'b str],
    ) -> Vec<(usize, String)> {
        accounts
            .iter()
            .enumerate()
            .map(|(idx, account)| (idx, format!("{}_{}", account, self.trait_method_name)))
            .collect()
    }

    pub(crate) fn args_tt(&self) -> TokenStream {
        let args = self.args.clone();
        quote!(#(#args),*)
    }
}

#[derive(Deserialize)]
pub(crate) struct FunctionSchema {
    pub(crate) name: String,
    pub(crate) kind: String,
    pub(crate) arguments: Vec<ArgumentSchema>,
}

impl FunctionSchema {
    pub(crate) fn metadata(&self) -> FunctionMetadata {
        FunctionMetadata::new(self.name.to_string(), self.arguments())
    }

    fn arguments(&self) -> Vec<FnArg> {
        self.arguments
            .iter()
            .map(|arg| syn::parse_str(&arg.argument()).unwrap())
            .collect()
    }
}

#[derive(Deserialize)]
pub(crate) struct ArgumentSchema {
    pub(crate) name: String,
    pub(crate) r#type: String,
}

impl ArgumentSchema {
    pub(crate) fn argument(&self) -> String {
        let r#type = self.r#type.replacen("String", "&str", 1);
        format!("{}: {}", self.name, r#type)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
}
