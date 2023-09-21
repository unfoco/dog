use crate::types;

mod misc;
mod board;

pub fn list() -> types::CommandVec {
    vec![misc::commands(), board::commands()]
        .into_iter()
        .flatten()
        .collect()
}
