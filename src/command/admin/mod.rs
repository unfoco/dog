use crate::types;

mod ban;
mod kick;
mod mute;
mod pin;
mod purge;
mod warn;

use ban::ban_user;
use ban::ban_message;
use kick::kick_user;
use kick::kick_message;
use mute::mute_user;
use mute::mute_message;
use purge::purge;
use purge::purge_message;
use warn::warn_user;
use warn::warn_message;
use pin::pin;

pub fn commands() -> types::CommandVec {
    return vec![
        ban_user(), ban_message(),
        kick_user(), kick_message(),
        mute_user(), mute_message(),
        purge(), purge_message(),
        warn_user(), warn_message(),
        pin(),
    ]
}
