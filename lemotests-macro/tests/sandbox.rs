use lemotests::{anyhow, tokio, Near, StateBuilder, Tgas, MARKET, NFT_PATH};
use lemotests_macro::add_helpers;

add_helpers!("./lemotests/nft_schema.json");

#[tokio::test]
async fn base_features_works() -> anyhow::Result<()> {
    let blockchain = StateBuilder::testnet()
        .with_contract("nft_token", NFT_PATH, Near(10))?
        .with_contract("market", MARKET, Near(20))?
        .with_alice(Near(20))?
        .with_bob(Near(10))?
        .build()
        .await?;

    let mut result = blockchain
        .alice_call_nft_token_init("alice")?
        .with_gas(Tgas(10))
        .then()
        .alice_call_nft_token_nft_transfer("alice", "1", None, None)?
        .with_deposit(100000000)
        .with_gas(100000000)
        .then()
        .alice_call_nft_token_mint("alice")?
        .with_deposit(Near(1))
        .with_gas(Tgas(5))
        .then()
        .alice_call_nft_token_nft_transfer("bob", "1", None, None)?
        .with_deposit(Near(1))
        .with_gas(Tgas(5))
        .then()
        .bob_view_nft_token_nft_token("1")
        .with_gas(Tgas(5))
        .execute()
        .await?;
    dbg!(result);

    Ok(())
}
