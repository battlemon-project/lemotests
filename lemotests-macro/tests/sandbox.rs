use lemotests::{anyhow, tokio, Near, StateBuilder, Tgas};
use lemotests_macro::add_helpers;

add_helpers!("./lemotests/nft_schema.json");

#[tokio::test]
async fn test1() -> anyhow::Result<()> {
    let blockchain = StateBuilder::new(lemotests::workspaces::testnet)
        .with_alice(Near(20))?
        .with_bob(Near(10))?
        .build()
        .await?;

    let result = blockchain
        .alice_call_nft_token_init("arg")?
        .with_deposit(Near(10))
        .with_gas(Tgas(10));
    Ok(())
}
