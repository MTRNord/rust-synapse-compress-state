use crate::StateMap;
use string_cache::{Atom, EmptyStaticAtomSet};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StateCompressorError {
    #[error("Missing state_goup: {}", .0)]
    MissingStateGroup(i64),
    #[error("Missing prev_state_group: {}", .0)]
    MissingPrevStateGroup(i64),
    #[error("States for group {} do not match. Expected {:#?}, found {:#?}", .0, .1, .2)]
    StateMissmatchedForGroup(
        i64,
        Box<StateMap<Atom<EmptyStaticAtomSet>>>,
        Box<StateMap<Atom<EmptyStaticAtomSet>>>,
    ),
}
