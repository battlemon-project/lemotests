pub struct Near(pub u128);

pub struct Tgas(pub u64);

pub trait Nearable {
    fn parse(&self) -> u128;
}

pub trait Gasable {
    fn parse(&self) -> u64;
}

impl Nearable for Near {
    fn parse(&self) -> u128 {
        self.0 * 10u128.pow(24)
    }
}

impl Nearable for u128 {
    fn parse(&self) -> u128 {
        *self
    }
}

impl Gasable for Tgas {
    fn parse(&self) -> u64 {
        self.0 * 10u64.pow(12)
    }
}

impl Gasable for u64 {
    fn parse(&self) -> u64 {
        *self
    }
}
