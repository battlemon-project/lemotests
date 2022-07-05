use core::fmt;
use std::error::Error;
use std::fmt::{Debug, Formatter};

#[derive(thiserror::Error)]
pub enum MacrosError {
    #[error("Failed to open file with json scheme. {0}")]
    FailedToOpenFileError(#[from] std::io::Error),
    #[error("Failed to deserialize json scheme. {0}")]
    DeserializeJsonSchemaError(#[from] serde_json::Error),
    #[error("Failed to parse. {0}")]
    FailedToParseError(#[from] syn::Error),
    #[error("Failed to destructure `Punctuated`: {0}")]
    DestructuringPunctuatedError(String),
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
