use crate::TxDetails;
use crate::{HelperError, State};
use indexmap::IndexMap;
use std::ops::Index;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum Key {
    Label(String),
    Index(usize),
}

#[derive(Debug)]
pub struct ChainResult<T> {
    tx_results: IndexMap<Key, TxDetails>,
    state: Option<State<T>>,
}

impl<T> ChainResult<T> {
    pub(crate) fn new() -> Self {
        Self {
            tx_results: IndexMap::new(),
            state: None,
        }
    }

    pub(crate) fn add_tx_details(&mut self, key: Key, tx_details: TxDetails) {
        self.tx_results.insert(key, tx_details);
    }

    pub fn tx(&self, label: impl AsRef<str>) -> Result<&TxDetails, HelperError> {
        self.tx_results
            .get(&Key::Label(label.as_ref().to_owned()))
            .ok_or_else(|| {
                HelperError::ChainResultError(format!("No tx with label {}", label.as_ref()))
            })
    }

    pub(crate) fn add_state(&mut self, state: State<T>) {
        self.state = Some(state);
    }

    pub fn into_state(self) -> State<T> {
        self.state.unwrap()
    }
}

impl<T> Index<usize> for ChainResult<T> {
    type Output = TxDetails;

    fn index(&self, index: usize) -> &Self::Output {
        self.tx_results
            .get_index(index)
            .map(|(_, v)| v)
            .unwrap_or_else(|| panic!("No tx with index {}", index))
    }
}

pub trait Contains {
    fn contains_error(&self, substring: &str) -> bool;
}

impl<T> Contains for Result<ChainResult<T>, HelperError> {
    fn contains_error(&self, substring: &str) -> bool {
        match self {
            Ok(_) => false,
            Err(e) => format!("{:?}", e).contains(substring),
        }
    }
}
