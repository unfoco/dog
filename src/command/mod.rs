use crate::types;

mod admin;

pub fn list() -> types::CommandVec {
    vec![admin::commands()].into_iter().flatten().collect()
}
