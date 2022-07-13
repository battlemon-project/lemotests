use std::fmt;
use std::ops::Sub;

#[derive(PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord, Debug)]
pub struct Near(pub u128);

impl Sub<Near> for Near {
    type Output = Near;

    fn sub(self, other: Near) -> Self::Output {
        Near(self.0.parse() - other.0.parse())
    }
}

impl Sub<u128> for Near {
    type Output = u128;

    fn sub(self, other: u128) -> Self::Output {
        self.parse() - other
    }
}

impl PartialEq<Near> for u128 {
    fn eq(&self, other: &Near) -> bool {
        self.eq(&other.parse())
    }
}

impl PartialEq<u128> for Near {
    fn eq(&self, other: &u128) -> bool {
        self.parse().eq(other)
    }
}

impl PartialOrd<Near> for u128 {
    fn partial_cmp(&self, other: &Near) -> Option<std::cmp::Ordering> {
        self.partial_cmp(&other.parse())
    }
}

impl PartialOrd<u128> for Near {
    fn partial_cmp(&self, other: &u128) -> Option<std::cmp::Ordering> {
        self.parse().partial_cmp(other)
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord, Debug)]
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
        write!(f, "{}", self.parse())
    }
}

impl fmt::Display for Tgas {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.parse())
    }
}
