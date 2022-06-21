use crate::{HelperError, TxWrapper};
use std::collections::BTreeMap;
use workspaces::{Account, Contract, DevNetwork, Worker};
pub type Accounts = BTreeMap<String, Account>;
pub type Contracts = BTreeMap<String, Contract>;

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
        if let Some(t) = self.tx_scenarios.as_mut() {
            t.push(tx)
        }
    }

    pub fn take_tx_scenarios(&mut self) -> Option<Vec<TxWrapper<T>>> {
        self.tx_scenarios.take()
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

    pub fn contract_key(&self, id: impl AsRef<str>) -> Option<&String> {
        self.contracts.get_key_value(id.as_ref()).map(|(k, _)| k)
    }

    pub fn account_key(&self, id: impl AsRef<str>) -> Option<&String> {
        self.accounts.get_key_value(id.as_ref()).map(|(k, _)| k)
    }

    pub fn alice(&self) -> Result<&Account, HelperError> {
        self.account("alice")
    }

    pub fn bob(&self) -> Result<&Account, HelperError> {
        self.account("bob")
    }
}
