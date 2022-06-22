use crate::{HelperError, Nearable, State};
use anyhow::Context;
use std::collections::BTreeMap;
use std::future::Future;
use std::path::PathBuf;
use workspaces::network::{DevAccountDeployer, Testnet};
use workspaces::types::Balance;
use workspaces::{Account, DevNetwork, Worker};

pub trait NearNetworks<F> {
    fn testnet(worker_fut: fn() -> F) -> Self;
}

pub struct StateBuilder<F> {
    worker_fut: fn() -> F,
    accounts: BTreeMap<String, Balance>,
    contracts: BTreeMap<String, (PathBuf, Balance)>,
}

impl<F> NearNetworks<F> for StateBuilder<F>
where
    F: Future<Output = anyhow::Result<Worker<Testnet>>>,
{
    fn testnet(worker_fut: fn() -> F) -> Self {
        Self {
            worker_fut,
            accounts: BTreeMap::new(),
            contracts: BTreeMap::new(),
        }
    }
}

impl<F, T> StateBuilder<F>
where
    F: Future<Output = anyhow::Result<Worker<T>>>,
    T: DevNetwork,
{
    pub fn new(worker_fut: fn() -> F) -> Self {
        Self {
            worker_fut,
            accounts: BTreeMap::new(),
            contracts: BTreeMap::new(),
        }
    }

    pub fn with_contract(
        mut self,
        id: &str,
        path: impl AsRef<std::path::Path>,
        balance: impl Nearable,
    ) -> Result<Self, HelperError> {
        self.contracts
            .try_insert(
                id.to_owned(),
                (path.as_ref().to_path_buf(), balance.parse()),
            )
            .map_err(|e| {
                HelperError::BuilderError(format!(
                    "Couldn't add task for contract creating with id `{}`",
                    e.entry.key()
                ))
            })?;

        Ok(self)
    }

    pub fn with_account<S: AsRef<str>>(
        mut self,
        id: S,
        balance: impl Nearable,
    ) -> Result<Self, HelperError> {
        self.accounts
            .try_insert(id.as_ref().to_owned(), balance.parse())
            .map_err(|e| {
                HelperError::BuilderError(format!(
                    "Couldn't add task for account creating with id `{}`",
                    e.entry.key()
                ))
            })?;
        Ok(self)
    }

    pub fn with_alice(self, balance: impl Nearable) -> Result<Self, HelperError> {
        self.with_account("alice", balance)
    }

    pub fn with_bob(self, balance: impl Nearable) -> Result<Self, HelperError> {
        self.with_account("bob", balance)
    }

    pub async fn build(self) -> Result<State<T>, HelperError> {
        let worker = (self.worker_fut)().await?;

        let root = worker
            .dev_create_account()
            .await
            .context("Failed to create root account while building")?;

        let (accounts, contracts) = self.process_accounts(&worker, &root).await?;

        Ok(State::new(root, worker, accounts, contracts, Vec::new()))
    }

    async fn process_accounts(
        self,
        worker: &Worker<T>,
        root: &Account,
    ) -> Result<(crate::Accounts, crate::Contracts), HelperError> {
        let mut accounts_buf = BTreeMap::new();
        let mut contracts_buf = BTreeMap::new();

        let accounts = self
            .accounts
            .iter()
            .chain(self.contracts.iter().map(|(k, v)| (k, &v.1)));

        for (id, balance) in accounts {
            let account = root
                .create_subaccount(worker, id)
                .initial_balance(*balance)
                .transact()
                .await?
                .into_result()?;

            if let Some((path, _)) = self.contracts.get(id) {
                let wasm = tokio::fs::read(path).await.map_err(|e| {
                    HelperError::BuilderError(format!(
                        "Failed to read contract bytes from file {e}",
                    ))
                })?;

                let contract = account.deploy(worker, &wasm).await?.into_result()?;
                contracts_buf.insert(id.to_owned(), contract);
                continue;
            }
            accounts_buf.insert(id.to_owned(), account);
        }

        Ok((accounts_buf, contracts_buf))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Near, NFT, NFT_PATH};

    #[test]
    fn builder_path_works() {
        StateBuilder::new(workspaces::testnet)
            .with_contract(NFT, NFT_PATH, Near(10))
            .unwrap();
    }

    #[test]
    fn builder_path_buf_works() {
        StateBuilder::new(workspaces::testnet)
            .with_contract(NFT, PathBuf::from(NFT_PATH), 10)
            .unwrap();
    }

    #[test]
    fn builder_ref_on_path_buf_works() {
        StateBuilder::new(workspaces::testnet)
            .with_contract(NFT, &PathBuf::from(NFT_PATH), 10)
            .unwrap();
    }

    #[test]
    fn builder_account_str_works() {
        StateBuilder::new(workspaces::testnet)
            .with_account("alice", 10)
            .unwrap();
    }

    #[test]
    fn builder_account_string_works() {
        StateBuilder::new(workspaces::testnet)
            .with_account(String::from("alice"), 10)
            .unwrap();
    }

    #[test]
    fn builder_account_ref_on_string_works() {
        StateBuilder::new(workspaces::testnet)
            .with_account(&String::from("alice"), 10)
            .unwrap();
    }
}
