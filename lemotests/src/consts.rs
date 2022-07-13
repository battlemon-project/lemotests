use workspaces::types::Balance;
pub const ALMOST_ZERO: Balance = 10_u128.pow(23); // 0.1 near
// pub const ONE_NEAR: Balance = parse_near!("1 N");
// pub const FOUR_NEAR: Balance = parse_near!("4 N");
// pub const FIVE_NEAR: Balance = parse_near!("5 N");
// pub const SIX_NEAR: Balance = parse_near!("6 N");
// pub const TEN_NEAR: Balance = parse_near!("10 N");
// pub const FIFTEEN_NEAR: Balance = parse_near!("15 N");
// pub const SIXTEEN_NEAR: Balance = parse_near!("16 N");
pub const ALICE: &str = "alice";
pub const BOB: &str = "bob";
pub const CHARLIE: &str = "charlie";
pub const DAVE: &str = "dave";
pub const EDWARD: &str = "edward";
pub const FRED: &str = "fred";

pub const ACCOUNTS: [&str; 6] = [ALICE, BOB, CHARLIE, DAVE, EDWARD, FRED];
