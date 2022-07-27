use crate::chain_result::ChainResult;
use crate::tx_details::TxDetails;
use crate::Key;
use crate::{Gasable, HelperError, Nearable, State};
use anyhow::Context;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::Debug;
use workspaces::DevNetwork;

#[derive(Debug, Copy, Clone)]
pub enum TxKind {
    AccountCallContract,
    View,
    SelfContractCall,
    ViewAccount,
}

#[derive(Debug)]
pub struct TxWrapper<T> {
    account: Option<String>,
    contract: Option<String>,
    function: String,
    arguments: serde_json::Map<String, Value>,
    near: Option<u128>,
    gas: Option<u64>,
    tx_kind: TxKind,
    state: Option<State<T>>,
    label: Option<Key>,
}

impl<T: DevNetwork + Debug> TxWrapper<T> {
    pub fn new(
        account: Option<String>,
        contract: Option<String>,
        function: String,
        arguments: serde_json::Map<String, Value>,
        tx_kind: TxKind,
        state: State<T>,
    ) -> Self {
        Self {
            account,
            contract,
            arguments,
            function,
            near: None,
            gas: None,
            tx_kind,
            state: Some(state),
            label: None,
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

    pub(crate) fn label(&self) -> Option<Key> {
        self.label.clone()
    }

    pub fn with_label(mut self, label: impl AsRef<str>) -> Self {
        self.label = Some(Key::Label(label.as_ref().to_owned()));
        self
    }

    pub fn and() -> Self {
        todo!()
    }

    pub async fn execute(self) -> Result<ChainResult<T>, HelperError> {
        let mut state = self.then();
        let mut ret = ChainResult::new();

        for (idx, tx) in state.take_tx_scenarios().iter().enumerate() {
            let label = tx.label().unwrap_or(Key::Index(idx));
            let tx_details = process_tx(tx, &state).await?;
            ret.add_tx_details(label, tx_details);
        }
        ret.add_state(state);
        Ok(ret)
    }
}

async fn process_tx<T: DevNetwork + Debug>(
    tx: &TxWrapper<T>,
    state: &State<T>,
) -> Result<TxDetails, HelperError> {
    let account = tx.account().and_then(|a| state.account(a).ok());
    let contract = tx.contract().and_then(|c| state.contract(c).ok());
    let tx_error = || format!("Failed to process transaction. Transaction details: {tx:?}");

    match tx.tx_kind {
        TxKind::ViewAccount => {
            let account = account.ok_or_else(|| {
                HelperError::TransactionError(
                    "the provided account hasn't found or doesn't exist in state.".to_owned(),
                )
            })?;

            let ret = account.view_account(state.worker()).await?;
            Ok(TxDetails::ViewAccount(ret))
        }

        TxKind::AccountCallContract => {
            let account = account.ok_or_else(|| {
                HelperError::TransactionError(
                    "the provided account hasn't found or doesn't exist in state.".to_owned(),
                )
            })?;

            let contract = contract.ok_or_else(|| {
                HelperError::TransactionError(
                    "the provided contract hasn't found or doesn't exist in state.".to_owned(),
                )
            })?;

            let ret = account
                .call(state.worker(), contract.id(), tx.function())
                .deposit(tx.near())
                .gas(tx.gas())
                .args_json(tx.arguments())
                .with_context(|| format!("Failed to parse JSON. Arguments {:?}", tx.arguments()))?
                .transact()
                .await
                .with_context(tx_error)?;

            Ok(TxDetails::Call(Box::new(ret)))
        }
        TxKind::View => {
            let contract = contract.ok_or_else(|| {
                HelperError::TransactionError(
                    "the provided contract hasn't found or doesn't exist in state.".to_owned(),
                )
            })?;

            let ret = contract
                .call(state.worker(), tx.function())
                .gas(tx.gas())
                .args_json(tx.arguments())
                .with_context(|| format!("Failed to parse JSON. Arguments {:?}", tx.arguments()))?
                .view()
                .await
                .with_context(tx_error)?;

            Ok(TxDetails::View(ret))
        }
        TxKind::SelfContractCall => {
            let contract = contract.ok_or_else(|| {
                HelperError::TransactionError(
                    "the provided contract hasn't found or doesn't exist in state.".to_owned(),
                )
            })?;

            let ret = contract
                .call(state.worker(), tx.function())
                .deposit(tx.near())
                .gas(tx.gas())
                .args_json(tx.arguments())
                .with_context(|| format!("Failed to parse JSON. Arguments {:?}", tx.arguments()))?
                .transact()
                .await
                .with_context(tx_error)?;

            Ok(TxDetails::Call(Box::new(ret)))
        }
    }
}
