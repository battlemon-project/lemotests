The thin wrapper around Near's `workspaces-rs` crate. It provides a simple way to create the "state" of blockchain for
testing purposes.

For comparison:

`workspaces-rs`

```rust
#[tokio::test]
async fn test_my_contract_with_workspaces() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let nft_wasm = tokio::fs::read("../nft_token.wasm").await?;
    let market_wasm = tokio::fs::read("../nft_market.wasm").await?;
    let root = worker.dev_create_account().await?;

    let nft = root
        .create_subaccount(&worker, "nft")
        .initial_balance(parse_near!("10 N"))
        .transact()
        .await?
        .into_result()?;

    let market = root
        .create_subaccount(&worker, "market")
        .initial_balance(parse_near!("10 N"))
        .transact()
        .await?
        .into_result()?;

    let market_contract = market.deploy(&worker, &market_wasm).await?.into_result()?;

    let nft_contract = nft.deploy(&worker, &nft_wasm).await?.into_result()?;

    let alice = root
        .create_subaccount(&worker, "alice")
        .initial_balance(parse_near!("10 N"))
        .transact()
        .await?
        .into_result()?;

    let bob = root
        .create_subaccount(&worker, "bob")
        .initial_balance(parse_near!("10 N"))
        .transact()
        .await?
        .into_result()?;

    market
        .call(&worker, market_contract.id(), "init")
        .gas(10_000_000_000_000)
        .args_json(serde_json::json!({
            "nft_id": nft_contract.id(),
        }))?
        .transact()
        .await?;

    nft.call(&worker, nft_contract.id(), "init")
        .gas(10_000_000_000_000)
        .args_json(serde_json::json!({
            "owner_id": nft_contract.id(),
        }))?
        .transact()
        .await?;

    alice
        .call(&worker, nft_contract.id(), "mint")
        .deposit(parse_near!("1 N"))
        .gas(10_000_000_000_000)
        .args_json(serde_json::json!({
            "account_id": alice.id(),
        }))?
        .transact()
        .await?;

    let msg = format!(
        "{{\"action\":\"add_ask\",\"price\":\"{}\"}}",
        parse_near!("5 N")
    );

    alice
        .call(&worker, nft_contract.id(), "nft_approve")
        .args_json(serde_json::json!({
            "token_id": "1",
            "account_id": market_contract.id(),
            "msg": msg
        }))?
        .gas(10_000_000_000_000)
        .deposit(parse_near!("1 N"))
        .transact()
        .await?;

    bob.call(&worker, market_contract.id(), "add_bid")
        .deposit(parse_near!("6 N"))
        .max_gas()
        .args_json(serde_json::json!({
            "token_id": "1",
        }))?
        .transact()
        .await?;

    let result = nft_contract.call(&worker, "nft_token").view().await?;
    let json: Value = result.json()?;
    assert_eq!(json["owner_id"].as_str().unwrap(), bob.id().as_str());

    let _alice_balance = alice.view_account(&worker).await?.balance;
    let _bob_balance = alice.view_account(&worker).await?.balance;

    Ok(())
}
```

`lemotests`

```rust
#[tokio::test]
async fn test_my_contract_with_lemotests() -> anyhow::Result<()> {
    let blockchain = StateBuilder::sandbox()
        .with_contract(NFT, NFT_PATH, Near(10))?
        .with_contract(MARKET, MARKET_PATH, Near(10))?
        .with_alice(Near(10))?
        .with_bob(Near(10))?
        .build()
        .await?;

    let [nft, market, alice, bob] = blockchain.string_ids()?;
    let msg = format!("{{\"action\":\"add_ask\",\"price\":\"{}\"}}", Near(5));

    let result = blockchain
        .call_nft_contract_init(&nft)?
        .with_gas(Tgas(10))
        .then()
        .call_market_contract_init(&nft)?
        .with_gas(Tgas(10))
        .then()
        .alice_call_nft_contract_nft_mint(&alice)?
        .with_deposit(Near(1))
        .with_gas(Tgas(10))
        .then()
        .alice_call_nft_contract_nft_approve("1", &market, Some(&msg))?
        .with_deposit(Near(1))
        .with_gas(Tgas(10))
        .then()
        .bob_call_market_contract_add_bid("1", None)?
        .with_deposit(Near(6))
        .with_gas(Tgas(200))
        .then()
        .view_nft_contract_nft_token("1")?
        .with_label("view_nft_token")
        .then()
        .view_account(ALICE)?
        .with_label("view_alice")
        .then()
        .view_account(BOB)?
        .with_label("view_bob")
        .execute()
        .await?;

    let nft_token = result.tx("view_nft_token")?.json::<TokenExt>()?;
    assert_eq!(nft_token.owner_id.as_str(), bob.as_str());

    let alice_balance = result.tx("view_alice")?.balance();
    let bob_balance = result.tx("view_bob")?.balance();

    Ok(())
}
```
