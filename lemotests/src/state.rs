use crate::{HelperError, TxDetails, TxKind, TxWrapper};
use core::fmt;
use std::fmt::Formatter;
use workspaces::{Account, AccountId, Contract, DevNetwork, Worker};
pub type Accounts = indexmap::IndexMap<String, Account>;
pub type Contracts = indexmap::IndexMap<String, Contract>;

pub struct State<T> {
    root: Account,
    worker: Worker<T>,
    accounts: Accounts,
    contracts: Contracts,
    tx_scenarios: Option<Vec<TxWrapper<T>>>,
}

impl<T> State<T>
where
    T: DevNetwork,
{
    pub(crate) fn new(
        root: Account,
        worker: Worker<T>,
        accounts: Accounts,
        contracts: Contracts,
        tx_scenarios: Vec<TxWrapper<T>>,
    ) -> Self {
        Self {
            root,
            worker,
            accounts,
            contracts,
            tx_scenarios: Some(tx_scenarios),
        }
    }

    pub fn add_tx_scenario(&mut self, tx: TxWrapper<T>) {
        self.tx_scenarios.get_or_insert(Vec::new()).push(tx);
    }

    pub fn take_tx_scenarios(&mut self) -> Vec<TxWrapper<T>> {
        self.tx_scenarios.take().unwrap_or_else(Vec::new)
    }

    pub fn worker(&self) -> &Worker<T> {
        &self.worker
    }

    pub fn root(&self) -> &Account {
        &self.root
    }

    pub fn account(&self, id: impl AsRef<str>) -> Result<&Account, HelperError> {
        self.accounts
            .get(id.as_ref())
            .ok_or_else(|| HelperError::AccountNotFound(id.as_ref().to_owned()))
    }

    pub fn contract(&self, id: impl AsRef<str>) -> Result<&Contract, HelperError> {
        self.contracts
            .get(id.as_ref())
            .ok_or_else(|| HelperError::ContractNotFound(id.as_ref().to_owned()))
    }

    pub fn contract_id(&self, id: impl AsRef<str>) -> Result<String, HelperError> {
        self.contract(id)
            .map(|contract| contract.id().as_str().to_owned())
    }

    pub fn contract_key(&self, id: impl AsRef<str>) -> Option<&String> {
        self.contracts.get_key_value(id.as_ref()).map(|(k, _)| k)
    }

    pub fn account_key(&self, id: impl AsRef<str>) -> Option<&String> {
        self.accounts.get_key_value(id.as_ref()).map(|(k, _)| k)
    }

    pub fn view_account(self, id: impl AsRef<str>) -> Result<TxWrapper<T>, HelperError> {
        let ret = TxWrapper::new(
            Some(id.as_ref().to_owned()),
            None,
            "view_balance".to_owned(),
            serde_json::Map::new(),
            TxKind::ViewAccount,
            self,
        );

        Ok(ret)
    }

    /// Returns ids for contracts and accounts.
    /// The order starts from contract ids and then accounts ids.
    /// The order of ids inside the group is the order of ids in your code.
    pub fn string_ids<const N: usize>(&self) -> Result<[String; N], HelperError> {
        self.contracts
            .values()
            .map(|contract| contract.id().as_str().to_owned())
            .chain(
                self.accounts
                    .values()
                    .map(|account| account.id().as_str().to_owned()),
            )
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|_| {
                HelperError::DestructuringError(
                    "The provided amount of accounts isn't valid for the current state".to_owned(),
                )
            })
    }

    pub fn alice(&self) -> Result<&Account, HelperError> {
        self.account("alice")
    }

    pub fn alice_id(&self) -> Result<String, HelperError> {
        self.alice().map(|account| account.id().as_str().to_owned())
    }

    pub fn bob(&self) -> Result<&Account, HelperError> {
        self.account("bob")
    }
}
