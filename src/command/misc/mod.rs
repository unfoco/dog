use crate::types;

mod ban;
mod help;
mod purge;
mod warn;

use ban::ban;
use ban::unban;
use help::help;
use purge::purge;
use warn::warn;

pub fn commands() -> types::CommandVec {
    return vec![ban(), unban(), help(), purge(), warn()]
}
