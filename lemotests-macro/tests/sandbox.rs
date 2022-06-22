use lemotests::{anyhow, tokio, Near, StateBuilder, Tgas, MARKET, NFT_PATH};
use lemotests_macro::add_helpers;

add_helpers!("./lemotests/nft_schema.json");

#[tokio::test]
async fn base_features_works() -> anyhow::Result<()> {
    let blockchain = StateBuilder::new(lemotests::workspaces::testnet)
        .with_contract("nft_token", NFT_PATH, Near(10))?
        .with_contract("market", MARKET, Near(20))?
        .with_alice(Near(20))?
        .with_bob(Near(10))?
        .build()
        .await?;

    let alice_id = blockchain.alice()?.id().to_string();

    let mut result = blockchain
        .alice_call_nft_token_init(&alice_id)?
        .with_gas(Tgas(10))
        .then()
        .alice_call_nft_token_nft_transfer(&alice_id, "1", None, None)?
        .with_deposit(100000000)
        .with_gas(100000000)
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
    dbg!(result);

    Ok(())
}
