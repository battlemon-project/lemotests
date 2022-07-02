use std::fmt::Debug;
use workspaces::result::{CallExecutionDetails, ExecutionOutcome, ViewResultDetails};

pub enum TxDetails {
    Call(Box<CallExecutionDetails>),
    View(ViewResultDetails),
}

impl TxDetails {
    pub fn json<T: serde::de::DeserializeOwned>(&self) -> anyhow::Result<T> {
        match self {
            TxDetails::Call(details) => details.json(),
            TxDetails::View(details) => details.json(),
        }
    }

    pub fn logs(&self) -> Vec<&str> {
        match self {
            TxDetails::Call(details) => details.logs(),
            _ => panic!("logs not available for view results"),
        }
    }

    pub fn outcome(&self) -> &ExecutionOutcome {
        match self {
            TxDetails::Call(details) => details.outcome(),
            _ => panic!("View result has no outcome"),
        }
    }

    pub fn outcomes(&self) -> Vec<&ExecutionOutcome> {
        match self {
            TxDetails::Call(details) => details.outcomes(),
            _ => panic!("View result has no outcomes"),
        }
    }

    pub fn is_success(&self) -> bool {
        match self {
            TxDetails::Call(details) => details.is_success(),
            _ => panic!("View result has no `is_success`"),
        }
    }
}

impl Debug for TxDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TxDetails::Call(details) => write!(f, "{:?}", details),
            TxDetails::View(details) => write!(
                f,
                "TxDetails::View(ViewResultDetails {{logs {:?}, result: {:?} }})",
                details.logs, details.result
            ),
        }
    }
}
