use lemotests::add_helpers;
use test_helpers::{anyhow, tokio, Near, StateBuilder};

add_helpers!("./nft_schema.json");

#[tokio::test]
async fn test1() -> anyhow::Result<()> {
    let blockchain = StateBuilder::new(test_helpers::workspaces::testnet)
    //     .with_alice(Near(20))?
    //     .with_bob(Near(10))?
        .build()
        .await?;
    // let b = None;
    // let test = a.or(b).ok_or_else(|| {
    //     test_helpers::HelperError::AccountAndContractNotFound(format!("{a:?}, {b:?}"))
    // })?;
    blockchain.alice_call_nft_token_init("");
    let result = blockchain.alice_call_nft_token_init("");
    assert!(result.is_err());
    // .with_deposit(Near(10))
    // .with_gas(Tgas(29))
    // .await?;

    Ok(())
}
