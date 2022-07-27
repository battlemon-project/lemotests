use std::fmt::Debug;
use workspaces::result::{CallExecutionDetails, ExecutionOutcome, ViewResultDetails};
use workspaces::AccountDetails;

#[derive(Debug)]
pub enum TxDetails {
    Call(Box<CallExecutionDetails>),
    View(ViewResultDetails),
    ViewAccount(AccountDetails),
}

impl TxDetails {
    pub fn balance(&self) -> u128 {
        match self {
            TxDetails::Call(_) => unimplemented!("balance method not available for `Call`"),
            TxDetails::View(_) => unimplemented!("balance method not available for `View`"),
            TxDetails::ViewAccount(details) => details.balance,
        }
    }

    pub fn json<T: serde::de::DeserializeOwned>(&self) -> anyhow::Result<T> {
        match self {
            TxDetails::Call(details) => details.json(),
            TxDetails::View(details) => details.json(),
            TxDetails::ViewAccount(_) => unimplemented!("json not available for ViewAccount"),
        }
    }

    pub fn logs(&self) -> Vec<&str> {
        match self {
            TxDetails::Call(details) => details.logs(),
            _ => unimplemented!("logs not available for view results"),
        }
    }

    pub fn outcome(&self) -> &ExecutionOutcome {
        match self {
            TxDetails::Call(details) => details.outcome(),
            _ => unimplemented!("View result has no outcome"),
        }
    }

    pub fn outcomes(&self) -> Vec<&ExecutionOutcome> {
        match self {
            TxDetails::Call(details) => details.outcomes(),
            _ => unimplemented!("View result has no outcomes"),
        }
    }

    pub fn is_success(&self) -> bool {
        match self {
            TxDetails::Call(details) => details.is_success(),
            _ => unimplemented!("View result has no `is_success`"),
        }
    }

    pub fn gas_used(&self) -> u64 {
        // match self {
        //     TxDetails::Call(details) => details.gas_used(),
        //     _ => unimplemented!("View result has no gas used"),
        // }

        todo!()
    }
}
