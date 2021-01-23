use structopt::StructOpt;

use crate::ServiceIdentifier;
use dsf_core::types::*;

/// PeerState defines the state of a peer
#[derive(Debug, Clone, StructOpt, PartialEq, Serialize, Deserialize, Display)]
pub enum SubscriberCommands {
    /// List all subscribers
    List,
}
