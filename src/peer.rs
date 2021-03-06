use std::net::SocketAddr;
use std::time::{Duration, SystemTime};

use structopt::StructOpt;

use dsf_core::types::*;

use crate::helpers::{parse_duration, try_parse_sock_addr};
use crate::ServiceIdentifier;

/// PeerState defines the state of a peer
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Display)]
pub enum PeerState {
    /// A peer that has not been contacted exists in the Unknown state
    Unknown,
    /// Once public keys have been exchanged this moves to the Known state
    Known(PublicKey),
    //Peered(Service),
}

/// PeerState defines the state of a peer
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PeerAddress {
    /// Implicit address
    Implicit(Address),
    /// Explicit / requested address
    Explicit(Address),
}

/// PeerInfo object for storage and exchange of peer information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "diesel", derive(diesel::Queryable))]
pub struct PeerInfo {
    pub id: Id,
    pub index: usize,
    pub address: PeerAddress,
    pub state: PeerState,
    pub seen: Option<SystemTime>,

    pub sent: u64,
    pub received: u64,
    pub blocked: bool,
}

impl PeerInfo {
    pub fn new(
        id: Id,
        address: PeerAddress,
        state: PeerState,
        index: usize,
        seen: Option<SystemTime>,
    ) -> Self {
        Self {
            id,
            address,
            state,
            seen,
            index,
            sent: 0,
            received: 0,
            blocked: false,
        }
    }

    /// Fetch the address of a peer
    pub fn address(&self) -> &Address {
        match &self.address {
            PeerAddress::Explicit(e) => e,
            PeerAddress::Implicit(i) => i,
        }
    }

    pub fn update_address(&mut self, addr: PeerAddress) {
        use PeerAddress::*;

        match (&self.address, &addr) {
            (_, Explicit(_)) => self.address = addr,
            (Implicit(_), Implicit(_)) => self.address = addr,
            _ => (),
        }
    }

    /// Fetch the state of a peer
    pub fn state(&self) -> &PeerState {
        &self.state
    }

    // Set the state of a peer
    pub fn set_state(&mut self, state: PeerState) {
        self.state = state;
    }

    pub fn seen(&self) -> Option<SystemTime> {
        self.seen
    }

    pub fn set_seen(&mut self, seen: SystemTime) {
        self.seen = Some(seen);
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, StructOpt)]
pub enum PeerCommands {
    #[structopt(name = "list")]
    /// List known peers
    List(PeerOptions),

    #[structopt(name = "connect")]
    /// Connects to a known peer
    Connect(ConnectOptions),

    #[structopt(name = "info")]
    /// Fetches information for a given peer
    Info(ServiceIdentifier),

    #[structopt(name = "search")]
    /// Searches the database for a peer
    Search(SearchOptions),

    #[structopt(name = "remove")]
    /// Removes a known peer from the database
    Remove(ServiceIdentifier),

    #[structopt(name = "block")]
    /// TODO: Blocks a peer
    Block(ServiceIdentifier),

    #[structopt(name = "unblock")]
    /// TODO: Unblocks a peer
    Unblock(ServiceIdentifier),
}

/// ConnectOptions passed to connect function
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, StructOpt)]
pub struct ConnectOptions {
    #[structopt(parse(try_from_str = try_parse_sock_addr))]
    /// Socket address for connection attempt
    pub address: SocketAddr,

    #[structopt(short = "i", long = "id")]
    /// ID of the remote node
    pub id: Option<Id>,

    #[structopt(short = "t", long = "timeout", parse(try_from_str = parse_duration))]
    /// ID of the remote node
    pub timeout: Option<Duration>,
}

// Peer list options
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, StructOpt)]
pub struct PeerOptions {}

/// ConnectOptions passed to connect function
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, StructOpt)]
pub struct SearchOptions {
    #[structopt(short = "i", long = "id")]
    /// ID of the peer to find
    pub id: Id,

    #[structopt(short = "t", long = "timeout", parse(try_from_str = parse_duration))]
    /// ID of the remote node
    pub timeout: Option<Duration>,
}

/// ConnectInfo returned by connect function
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConnectInfo {
    pub id: Id,
    pub peers: usize,
}
