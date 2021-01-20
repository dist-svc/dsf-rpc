use structopt::StructOpt;

use dsf_core::types::*;
use crate::ServiceIdentifier;

/// PeerState defines the state of a peer
#[derive(Debug, Clone, StructOpt, PartialEq, Serialize, Deserialize, Display)]
pub enum SubscriberCommands {
    /// List all subscribers
    List,
}

