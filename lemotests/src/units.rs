use std::fmt;
use std::ops::Sub;

pub struct Near(pub u128);

impl Sub<u128> for Near {
    type Output = u128;

    fn sub(self, other: u128) -> Self::Output {
        self.parse() - other
    }
}

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

impl fmt::Display for Near {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for Tgas {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
