use crate::{Bridge, BridgeService};
use tron_macros::auto_impl_bridge;

pub mod balance;
pub mod bug;
pub mod chat;
pub mod friends;
pub mod leaderboards;
pub mod perms;
pub mod players;
pub mod prefix;
pub mod proxy;
pub mod redeem;
pub mod report;
pub mod scoreboard;
pub mod servers;
pub mod session;
pub mod shop;
pub mod team;

#[auto_impl_bridge("../tron-protos/proto/bridge.proto")]
impl Bridge for BridgeService {}
