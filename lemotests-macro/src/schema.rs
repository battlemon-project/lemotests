use crate::MacrosError;
use lemotests::TxKind;
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
    pub(crate) fn blueprints(&self, accounts: &[&str]) -> Vec<FunctionBlueprint> {
        self.functions
            .iter()
            .flat_map(|f| f.blueprints(self.name.clone(), accounts))
            .collect()
    }
}

pub(crate) struct FunctionBlueprint {
    pub(crate) contract_function_name: String,
    pub(crate) trait_method_name: String,
    pub(crate) args: Vec<FnArg>,
    pub(crate) account: Option<String>,
    pub(crate) tx_kind: TxKind,
    pub(crate) contract_name: String,
}

impl FunctionBlueprint {
    pub(crate) fn new(
        contract_function_name: String,
        trait_method_name: String,
        args: Vec<FnArg>,
        account: Option<String>,
        tx_kind: TxKind,
        contract_name: String,
    ) -> Self {
        Self {
            contract_function_name: contract_function_name.clone(),
            trait_method_name,
            args,
            account,
            tx_kind,
            contract_name,
        }
    }

    pub fn tx_kind(&self) -> &TxKind {
        &self.tx_kind
    }

    pub fn args_without_types(&self) -> Vec<&PatIdent> {
        self.args
            .iter()
            .map(|arg| {
                let FnArg::Typed(pat) = arg else { unreachable!() };
                let Pat::Ident(ident) = &*pat.pat else { unreachable!() };
                ident
            })
            .collect()
    }

    pub fn args_tt(&self) -> TokenStream {
        let args = self.args.clone();
        quote!(#(#args),*)
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FunctionKind {
    Call,
    View,
}

#[derive(Deserialize)]
pub(crate) struct FunctionSchema {
    pub(crate) name: String,
    kind: FunctionKind,
    pub(crate) arguments: Vec<ArgumentSchema>,
}

impl FunctionSchema {
    pub fn kind(&self) -> &FunctionKind {
        &self.kind
    }

    pub(crate) fn blueprints(
        &self,
        contract_name: String,
        accounts: &[&str],
    ) -> Vec<FunctionBlueprint> {
        let arguments = self.arguments();
        let mut ret = Vec::new();
        let contract_function_name = &self.name;
        match self.kind {
            FunctionKind::Call => {
                let self_contract_call = FunctionBlueprint::new(
                    contract_function_name.clone(),
                    format!("call_{contract_name}_{contract_function_name}"),
                    arguments.clone(),
                    None,
                    TxKind::SelfContractCall,
                    contract_name.clone(),
                );
                ret.push(self_contract_call);

                for account in accounts {
                    let blueprint = FunctionBlueprint::new(
                        contract_function_name.clone(),
                        format!("{account}_call_{contract_name}_{contract_function_name}"),
                        arguments.clone(),
                        Some(account.to_string()),
                        TxKind::AccountCallContract,
                        contract_name.clone(),
                    );
                    ret.push(blueprint);
                }
                ret
            }
            FunctionKind::View => {
                let blueprint = FunctionBlueprint::new(
                    contract_function_name.clone(),
                    format!("view_{contract_name}_{contract_function_name}"),
                    arguments,
                    None,
                    TxKind::View,
                    contract_name,
                );
                vec![blueprint]
            }
        }
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
