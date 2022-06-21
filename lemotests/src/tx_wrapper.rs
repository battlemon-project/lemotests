use crate::workspaces::{Account, Contract};
use crate::{Gasable, Nearable};
use near_sdk::serde::Serialize;
pub struct TxWrapper<'a, T, V> {
    account: Option<&'a Account>,
    contract: Option<&'a Contract>,
    arguments: T,
    function: V,
    near: Option<u128>,
    gas: Option<u64>,
}

impl<'a, T, V> TxWrapper<'a, T, V>
where
    T: Serialize,
    V: AsRef<str>,
{
    pub fn new(
        account: Option<&'a Account>,
        contract: Option<&'a Contract>,
        function: V,
        arguments: T,
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
