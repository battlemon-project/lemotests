use lemotests::{Callable, StateBuilder, NFT_PATH, TEN_NEAR};

#[tokio::test]
async fn builder_works() -> Result<(), anyhow::Error> {
    let state = StateBuilder::new(workspaces::testnet)
        .with_alice(TEN_NEAR)?
        .with_bob(TEN_NEAR)?
        .with_contract("nft", NFT_PATH, TEN_NEAR)?
        .build()
        .await?;

    let _ = state.call(("alice", "nft", "nft_mint"))?.transact();

    Ok(())
}
