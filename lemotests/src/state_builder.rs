use crate::{HelperError, Nearable, State, ALICE, BOB, CHARLIE, DAVE, EDWARD, FRED};
use anyhow::Context;
use indexmap::IndexMap;
use std::fmt::Debug;
use std::future::Future;
use std::path::PathBuf;
use workspaces::network::{DevAccountDeployer, Sandbox, Testnet};
use workspaces::types::Balance;
use workspaces::{Account, DevNetwork, Worker};

pub struct StateBuilder<F> {
    worker_fut: fn() -> F,
    accounts: IndexMap<String, Balance>,
    contracts: IndexMap<String, (PathBuf, Balance)>,
}

impl StateBuilder<()> {
    pub fn testnet() -> StateBuilder<impl Future<Output = anyhow::Result<Worker<Testnet>>>> {
        StateBuilder::new(workspaces::testnet)
    }

    pub fn sandbox() -> StateBuilder<impl Future<Output = anyhow::Result<Worker<Sandbox>>>> {
        StateBuilder::new(workspaces::sandbox)
    }
}

impl<F, T> StateBuilder<F>
where
    F: Future<Output = anyhow::Result<Worker<T>>>,
    T: DevNetwork + Debug,
{
    pub fn new(worker_fut: fn() -> F) -> Self {
        Self {
            worker_fut,
            accounts: IndexMap::new(),
            contracts: IndexMap::new(),
        }
    }

    pub fn with_contract(
        mut self,
        id: impl AsRef<str>,
        path: impl AsRef<std::path::Path>,
        balance: impl Nearable,
    ) -> Result<Self, HelperError> {
        self.contracts
            .insert(
                id.as_ref().to_owned(),
                (path.as_ref().to_path_buf(), balance.parse()),
            )
            .map_or(Ok(self), |_| {
                Err(HelperError::BuilderError(format!(
                    "Couldn't add task for contract with id `{}` because it already exists",
                    id.as_ref().to_owned()
                )))
            })
    }

    pub fn with_account(
        mut self,
        id: impl AsRef<str>,
        balance: impl Nearable,
    ) -> Result<Self, HelperError> {
        self.accounts
            .insert(id.as_ref().to_owned(), balance.parse())
            .map_or(Ok(self), |_| {
                Err(HelperError::BuilderError(format!(
                    "Couldn't add task for account with id `{}` because it already exists",
                    id.as_ref().to_owned()
                )))
            })
    }

    pub fn with_alice(self, balance: impl Nearable) -> Result<Self, HelperError> {
        self.with_account(ALICE, balance)
    }

    pub fn with_bob(self, balance: impl Nearable) -> Result<Self, HelperError> {
        self.with_account(BOB, balance)
    }

    pub fn with_charlie(self, balance: impl Nearable) -> Result<Self, HelperError> {
        self.with_account(CHARLIE, balance)
    }

    pub fn with_dave(self, balance: impl Nearable) -> Result<Self, HelperError> {
        self.with_account(DAVE, balance)
    }

    pub fn with_edward(self, balance: impl Nearable) -> Result<Self, HelperError> {
        self.with_account(EDWARD, balance)
    }

    pub fn with_fred(self, balance: impl Nearable) -> Result<Self, HelperError> {
        self.with_account(FRED, balance)
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
        let mut accounts_buf = IndexMap::new();
        let mut contracts_buf = IndexMap::new();

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
    use crate::Near;
    use workspaces::testnet;
    const NFT: &str = "nft";
    const NFT_PATH: &str = "../../contract.wasm";

    #[test]
    fn builder_path_works() {
        StateBuilder::new(testnet)
            .with_contract(NFT, NFT_PATH, Near(10))
            .unwrap();
    }

    #[test]
    fn builder_path_buf_works() {
        StateBuilder::new(testnet)
            .with_contract(NFT, PathBuf::from(NFT_PATH), 10)
            .unwrap();
    }

    #[test]
    fn builder_ref_on_path_buf_works() {
        StateBuilder::new(testnet)
            .with_contract(NFT, &PathBuf::from(NFT_PATH), 10)
            .unwrap();
    }

    #[test]
    fn builder_account_str_works() {
        StateBuilder::new(testnet)
            .with_account("alice", 10)
            .unwrap();
    }

    #[test]
    fn builder_account_string_works() {
        StateBuilder::new(testnet)
            .with_account(String::from("alice"), 10)
            .unwrap();
    }

    #[test]
    fn builder_account_ref_on_string_works() {
        StateBuilder::new(testnet)
            .with_account(&String::from("alice"), 10)
            .unwrap();
    }
}
