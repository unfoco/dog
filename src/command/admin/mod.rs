use crate::types;

mod ban;
mod kick;
mod mute;
mod pin;
mod purge;
mod say;
mod warn;

pub fn commands() -> types::CommandVec {
    return vec![
        ban::ban_user(),
        ban::ban_message(),
        kick::kick_user(),
        kick::kick_message(),
        mute::mute_user(),
        mute::mute_message(),
        purge::purge(),
        purge::purge_message(),
        say::say(),
        warn::warn_user(),
        warn::warn_message(),
        warn::unwarn_user(),
        warn::unwarn_message(),
        pin::pin(),
    ];
}
