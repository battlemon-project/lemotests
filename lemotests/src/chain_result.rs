use crate::TxDetails;
use crate::{HelperError, State};
use indexmap::IndexMap;
use std::fmt;
use std::fmt::Formatter;
use std::ops::Index;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum Key {
    Label(String),
    Index(usize),
}

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

impl<T> fmt::Debug for ChainResult<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "tx_results: {:?}", self.tx_results)
    }
}
