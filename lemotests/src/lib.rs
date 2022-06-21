#![feature(map_try_insert)]

mod consts;
mod contract_initializer;
mod errors;
mod state;
mod state_builder;
mod tx_wrapper;
mod units;

pub use anyhow;
pub use consts::*;
pub use errors::*;
use near_sdk::test_utils::VMContextBuilder;
use near_sdk::AccountId;
pub use near_units::{parse_gas, parse_near};
use nft_models::lemon::Lemon;
use nft_models::ModelKind;
pub use serde_json;
pub use state::*;
pub use state_builder::*;
use token_metadata_ext::*;
pub use tokio;
pub use tx_wrapper::*;
pub use units::*;
pub use workspaces;
// pub fn fake_metadata_with<T>(model: T) -> TokenMetadataExt
// where
//     T: Manager + Into<ModelKind>,
// {
//     TokenMetadataExt {
//         title: Some("fake title".into()),
//         description: Some("this is fake description".into()),
//         media: Some("https://fakelinktomedia.com".into()),
//         media_hash: Some(vec![0, 1, 2, 3, 4].into()),
//         copies: None,
//         issued_at: None,
//         expires_at: None,
//         starts_at: None,
//         updated_at: None,
//         extra: None,
//         reference: None,
//         reference_hash: None,
//         model: model.into(),
//     }
// }

pub fn sample_token_metadata() -> TokenMetadataExt {
    let model: ModelKind = ModelKind::Lemon(get_foo_lemon());

    TokenMetadataExt {
        title: Some("foo title".into()),
        description: Some("this is description for foo title's token".into()),
        media: None,
        media_hash: None,
        copies: Some(1),
        issued_at: None,
        expires_at: None,
        starts_at: None,
        updated_at: None,
        extra: None,
        reference: None,
        reference_hash: None,
        model,
    }
}

pub fn get_foo_lemon() -> Lemon {
    use nft_models::lemon::*;

    Lemon::from_random(&[10, 11, 12, 14])
}

// pub fn get_foo_weapon() -> Weapon {
//     use nft_models::weapon::*;
//
//     Weapon {
//         level: 0,
//         r#type: Type::Instant,
//         parent: None,
//         slots: HashSet::new(),
//     }
// }
//
// pub fn foo_token_metadata_ext() -> TokenMetadataExt {
//     use nft_models::weapon::*;
//     let model: ModelKind = Weapon {
//         level: 0,
//         r#type: Type::Instant,
//         parent: None,
//         slots: HashSet::new(),
//     }
//     .into();
//
//     TokenMetadataExt {
//         title: Some("foo_token".into()),
//         description: Some("this is description for foo title's token".into()),
//         media: Some("link to media".into()),
//         media_hash: Some(vec![0, 1, 2, 3, 4].into()),
//         copies: Some(1),
//         issued_at: None,
//         expires_at: None,
//         starts_at: None,
//         updated_at: None,
//         extra: Some(
//             json!({
//                 "some": "json",
//                 "values": 9,
//             })
//             .to_string(),
//         ),
//         reference: None,
//         reference_hash: None,
//         model,
//     }
// }
//
// pub fn baz_token_metadata_ext() -> TokenMetadataExt {
//     use nft_models::lemon::*;
//     let model: ModelKind = Lemon {
//         option: Exo::MA01,
//         cap: Cap::Otherworldly,
//         r#type: Cloth::Heavy,
//         eyes: Eyes::Nakamoto,
//         background: Head::Red,
//         top: Teeth::Classical,
//         cyber_suit: CyberSuit::Black,
//         expression: Expression::Angry,
//         eyes: Eyes::Close,
//         hair: Hair::Punkkez,
//         accessory: Accessory::Toothpick,
//         winrate: Some(33),
//         rarity: 88,
//         teeth: None,
//         slots: HashSet::new(),
//     }
//     .into();
//
//     TokenMetadataExt {
//         title: Some("baz_token".into()),
//         description: Some("this is description for baz title's token".into()),
//         media: Some("link to media".into()),
//         media_hash: Some(vec![2, 3, 4, 3, 4].into()),
//         copies: Some(1),
//         issued_at: None,
//         expires_at: None,
//         starts_at: None,
//         updated_at: None,
//         extra: Some(
//             json!({
//                 "rust": "bless you!",
//                 "values": 2,
//             })
//             .to_string(),
//         ),
//         reference: None,
//         reference_hash: None,
//         model,
//     }
// }

pub fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
    let mut builder = VMContextBuilder::new();
    builder
        .current_account_id(AccountId::new_unchecked("alice".to_string()))
        .signer_account_id(predecessor_account_id.clone())
        .predecessor_account_id(predecessor_account_id);
    builder
}
