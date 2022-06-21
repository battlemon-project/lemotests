use crate::workspaces::{Account, Contract};
use crate::{Gasable, HelperError, Nearable, State};
use anyhow::Context;
use near_sdk::serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::Debug;
use workspaces::result::CallExecutionDetails;
use workspaces::DevNetwork;

pub struct TxWrapper<T> {
    account: Option<String>,
    contract: Option<String>,
    function: String,
    arguments: serde_json::Map<String, Value>,
    near: Option<u128>,
    gas: Option<u64>,
    state: Option<State<T>>,
}

impl<T: DevNetwork> TxWrapper<T> {
    pub fn new(
        account: Option<String>,
        contract: Option<String>,
        function: String,
        arguments: serde_json::Map<String, Value>,
        state: State<T>,
    ) -> Self {
        Self {
            account,
            contract,
            arguments,
            function,
            near: None,
            gas: None,
            state: Some(state),
        }
    }

    pub fn account(&self) -> Option<&String> {
        self.account.as_ref()
    }

    pub fn contract(&self) -> Option<&String> {
        self.contract.as_ref()
    }

    pub fn arguments(&self) -> &(impl Deserialize + Serialize + Debug) {
        &self.arguments
    }

    pub fn function(&self) -> &str {
        self.function.as_str()
    }

    pub fn gas(&self) -> u64 {
        self.gas.unwrap_or(0)
    }

    pub fn near(&self) -> u128 {
        self.near.unwrap_or(0)
    }

    pub fn with_deposit(mut self, deposit: impl Nearable) -> Self {
        self.near = Some(deposit.parse());
        self
    }

    pub fn with_gas(mut self, gas: impl Gasable) -> Self {
        self.gas = Some(gas.parse());
        self
    }

    pub fn then(mut self) -> State<T> {
        let mut state = self.state.take().unwrap();
        state.add_tx_scenario(self);
        state
    }

    pub fn and() -> Self {
        todo!()
    }

    pub async fn execute(self) -> Vec<Result<CallExecutionDetails, HelperError>> {
        let mut state = self.then();
        let mut buf = Vec::new();
        for tx in state.take_tx_scenarios().unwrap() {
            let tx_result = process_tx(tx, &state).await;
            buf.push(tx_result);
        }

        buf
    }
}

async fn process_tx<T: DevNetwork>(
    tx: TxWrapper<T>,
    state: &State<T>,
) -> Result<CallExecutionDetails, HelperError> {
    let account = tx.account().and_then(|a| state.account(a).ok());
    let contract = tx.contract().and_then(|c| state.contract(c).ok());

    let result = match (account, contract) {
        (Some(account), Some(contract)) => account
            .call(state.worker(), contract.id(), tx.function())
            .deposit(tx.near())
            .gas(tx.gas())
            .args_json(tx.arguments())
            .with_context(|| format!("Failed to parse JSON. Arguments {:?}", tx.arguments()))?
            .transact()
            .await
            .context("Failed to process transaction.")?,
        (None, Some(contract)) => {
            todo!("view method")
        }
        (Some(account), None) => {
            todo!("account call methods like transfer tokens or view state")
        }
        _ => unreachable!(),
    };

    Ok(result)
}
