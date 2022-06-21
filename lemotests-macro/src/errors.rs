use core::fmt;
use std::error::Error;
use std::fmt::{Debug, Formatter};

#[derive(thiserror::Error)]
pub enum MacrosError {
    #[error("Failed to open file with json scheme. {0}")]
    FailedToOpenFile(#[from] std::io::Error),
    #[error("Failed to deserialize json scheme. {0}")]
    DeserializeJsonSchema(#[from] serde_json::Error),
}

impl Debug for MacrosError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        error_chain_fmt(self, f)
    }
}

fn error_chain_fmt(error: &impl Error, f: &mut Formatter) -> fmt::Result {
    writeln!(f, "{}\n", error)?;
    let mut current = error.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }

    Ok(())
}
