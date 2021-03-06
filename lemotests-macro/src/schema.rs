use crate::blueprint::FunctionBlueprint;
use crate::MacrosError;
use lemotests::TxKind;
use serde::Deserialize;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use syn::FnArg;

pub(crate) fn read_json_schemas_from_file<P>(
    paths: &[P],
) -> Result<Vec<BufReader<File>>, MacrosError>
where
    P: AsRef<Path>,
{
    let mut ret = Vec::new();
    for path in paths {
        let file = File::open(path)?;
        ret.push(BufReader::new(file))
    }

    Ok(ret)
}

pub(crate) fn deserialize_json_schemas<R: Read>(
    readers: &mut [BufReader<R>],
) -> Result<Vec<ContractSchema>, MacrosError> {
    let mut ret = Vec::new();
    for reader in readers {
        let schema =
            serde_json::from_reader(reader).map_err(MacrosError::DeserializeJsonSchemaError)?;
        ret.push(schema)
    }
    Ok(ret)
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
