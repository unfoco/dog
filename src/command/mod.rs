use crate::types;

mod admin;

pub fn list() -> Vec<types::Command> {
    vec![
        admin::commands()
    ]
        .into_iter()
        .flatten()
        .collect()
}
