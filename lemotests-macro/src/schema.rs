use crate::MacrosError;
use serde::Deserialize;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

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
    serde_json::from_reader(reader).map_err(MacrosError::DeserializeJsonSchema)
}

#[derive(Deserialize)]
pub(crate) struct ContractSchema {
    pub(crate) name: String,
    pub(crate) functions: Vec<FunctionSchema>,
}

#[derive(Deserialize)]
pub(crate) struct FunctionSchema {
    pub(crate) name: String,
    pub(crate) kind: String,
    pub(crate) arguments: Vec<ArgumentSchema>,
}

#[derive(Deserialize)]
pub(crate) struct ArgumentSchema {
    pub(crate) name: String,
    pub(crate) r#type: String,
}
