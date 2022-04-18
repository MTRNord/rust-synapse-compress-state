use crate::StateMap;
use string_cache::{Atom, EmptyStaticAtomSet};
use thiserror::Error;
#[derive(Error, Debug)]
pub enum StateCompressorError {
    #[error("Missing state_goup: {0}")]
    MissingStateGroup(i64),
    #[error("Missing prev_state_group: {0}")]
    MissingPrevStateGroup(i64),
    #[error("States for group {0} do not match. Expected {1:#?}, found {2:#?}")]
    StateMissmatchedForGroup(
        i64,
        Box<StateMap<Atom<EmptyStaticAtomSet>>>,
        Box<StateMap<Atom<EmptyStaticAtomSet>>>,
    ),
    // This recursion is totally safe as we never have more than 2 levels of recursion
    #[error("expected state to match: {0}")]
    ExpectedStateMismatched(Box<StateCompressorError>),
}
