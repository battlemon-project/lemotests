use crate::helper::Helper;
use lemotests::{anyhow, tokio, Near, StateBuilder, Tgas, NFT, NFT_PATH};
use lemotests_macro::add_helpers;

// add_helpers!("./lemotests/nft_schema.json");
mod helper {
    use lemotests::{HelperError, TxWrapper};

    pub trait Helper<T> {
        fn alice_call_nft_token_init(
            self,
            owner_id: &str,
        ) -> Result<lemotests::TxWrapper<T>, lemotests::HelperError>;

        fn alice_call_nft_token_nft_transfer(
            self,
            receiver_id: &str,
            token_id: &str,
            approval_id: u64,
            memo: &str,
        ) -> Result<lemotests::TxWrapper<T>, lemotests::HelperError>;
    }

    impl<T> Helper<T> for lemotests::State<T>
    where
        T: lemotests::workspaces::DevNetwork,
    {
        fn alice_call_nft_token_init(self, owner_id: &str) -> Result<TxWrapper<T>, HelperError> {
            let account = self.account_key("alice").cloned();
            let contract = self.contract_key("nft_token").cloned();
            if account.is_none() && contract.is_none() {
                return Err(lemotests::HelperError::AccountAndContractNotFound(
                    "Failed to get account: `{#name}` and contract `#contract`".to_owned(),
                ));
            };

            let mut json_args = serde_json::Map::new();
            json_args.insert("owner_id".into(), owner_id.into());

            let tx = lemotests::TxWrapper::new(
                account,
                contract,
                "nft_transfer".to_owned(),
                json_args,
                self,
            );

            Ok(tx)
        }

        fn alice_call_nft_token_nft_transfer(
            self,
            receiver_id: &str,
            token_id: &str,
            approval_id: u64,
            memo: &str,
        ) -> Result<TxWrapper<T>, HelperError> {
            let mut json_args = serde_json::Map::new();
            json_args.insert("receiver_id".into(), receiver_id.into());
            json_args.insert("token_id".into(), token_id.into());
            json_args.insert("approval_id".into(), approval_id.into());
            json_args.insert("memo".into(), memo.into());
            let account = self.account_key("alice").cloned();
            let contract = self.contract_key("nft_token").cloned();

            if account.is_none() && contract.is_none() {
                return Err(lemotests::HelperError::AccountAndContractNotFound(todo!()));
            };

            let tx = lemotests::TxWrapper::new(
                account,
                contract,
                "nft_transfer".to_owned(),
                json_args,
                self,
            );
            Ok(tx)
        }
    }
}

#[tokio::test]
async fn test1() -> anyhow::Result<()> {
    let blockchain = StateBuilder::new(lemotests::workspaces::testnet)
        .with_contract("nft_token", NFT_PATH, Near(10))?
        .with_alice(Near(20))?
        .with_bob(Near(10))?
        .build()
        .await?;

    let alice_id = blockchain.alice()?.id().to_string();

    let mut result = blockchain
        .alice_call_nft_token_init(&alice_id)?
        .with_gas(Tgas(10))
        // .then()
        // .alice_call_nft_token_nft_transfer(&alice_id, "1", 0, "")?
        // .then()
        // .alice_call_nft_token_init("")?
        // .with_deposit(1000_0000)
        // .with_gas(10000)
        .execute()
        .await;
    //     .alice_call_nft_token_mint(blockchain.alice())
    //     .with_deposit(Near(1))
    //     .with_gas(Tgas(5))
    //     .then()
    //     .alice_call_nft_transfer(blockchain.bob(), "1", Some(10), "json string here")
    //     .with_deposit(Near(1))
    //     .with_gas(Tgas(5))
    //     .then()
    //     .view_nft_token("1")
    //     .with_gas(Tgas(5))
    //     .execute()
    //     .await?;
    let bla = result.pop().unwrap().unwrap();
    dbg!(bla);

    Ok(())
}
