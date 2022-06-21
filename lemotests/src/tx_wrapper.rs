use crate::workspaces::{Account, Contract};
use crate::{Gasable, Nearable};
use serde_json::Value;

pub struct TxWrapper<'a> {
    account: Option<&'a Account>,
    contract: Option<&'a Contract>,
    function: &'a str,
    arguments: serde_json::Map<String, Value>,
    near: Option<u128>,
    gas: Option<u64>,
}

impl<'a> TxWrapper<'a> {
    pub fn new(
        account: Option<&'a Account>,
        contract: Option<&'a Contract>,
        function: &'a str,
        arguments: serde_json::Map<String, Value>,
    ) -> Self {
        Self {
            account,
            contract,
            arguments,
            function,
            near: None,
            gas: None,
        }
    }

    pub fn with_deposit(mut self, deposit: impl Nearable) -> Self {
        self.near = Some(deposit.parse());
        self
    }

    pub fn with_gas(mut self, gas: impl Gasable) -> Self {
        self.gas = Some(gas.parse());
        self
    }

    pub fn send(&self) -> Result<(), String> {
        // self.account.send(self.near, self.gas)
        todo!()
    }
}
