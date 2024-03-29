use std::time::SystemTime;

extern crate chrono;
extern crate chrono_english;
extern crate chrono_humanize;
extern crate humantime;

extern crate futures;
use dsf_core::api::ServiceHandle;
use dsf_core::wire::Container;
use futures::prelude::*;
extern crate structopt;
use structopt::StructOpt;

extern crate strum;

#[macro_use]
extern crate strum_macros;

extern crate rand;
use rand::random;

extern crate dsf_core;
use dsf_core::error::Error;
use dsf_core::types::*;

#[macro_use]
extern crate serde;

extern crate colored;

pub mod config;
pub use config::*;

pub mod data;
pub use data::*;

pub mod debug;
pub use debug::*;

pub mod peer;
pub use peer::*;

pub mod service;
pub use service::*;

pub mod replica;
pub use replica::*;

pub mod subscriber;
pub use subscriber::*;

pub mod page;
pub use page::*;

pub mod name;
pub use name::*;

pub mod display;

mod helpers;

/// API trait implements RPC API for the daemon (or delegation)
pub trait Rpc {
    type Error;

    fn exec(&mut self, req: Request) -> Box<dyn Future<Output = Result<Response, Self::Error>>>;
}

/// RPC Request container for requests from a client to the daemon
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Request {
    req_id: u64,
    kind: RequestKind,
}

impl Request {
    pub fn new(kind: RequestKind) -> Self {
        Self {
            req_id: random(),
            kind,
        }
    }

    pub fn req_id(&self) -> u64 {
        self.req_id
    }

    pub fn kind(&self) -> RequestKind {
        self.kind.clone()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, StructOpt)]
pub struct ServiceIdentifier {
    #[structopt(short = "i", long = "id", group = "identifier")]
    /// Global service ID
    pub id: Option<Id>,

    #[structopt(short = "n", long = "index", group = "identifier")]
    /// Local service index
    pub index: Option<usize>,
}

impl ServiceIdentifier {
    pub fn id(id: Id) -> Self {
        Self {
            id: Some(id),
            index: None,
        }
    }

    pub fn index(index: usize) -> Self {
        Self {
            id: None,
            index: Some(index),
        }
    }
}

impl From<ServiceHandle> for ServiceIdentifier {
    fn from(h: ServiceHandle) -> Self {
        Self::id(h.id)
    }
}

impl From<Id> for ServiceIdentifier {
    fn from(id: Id) -> Self {
        Self::id(id)
    }
}

/// Paginator object supports paginating responses from the daemon
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, StructOpt)]
pub struct PageBounds {
    #[structopt(long = "count")]
    /// Maximum number of responses to return
    pub count: Option<usize>,

    #[structopt(long = "offset")]
    /// Offset of returned results
    pub offset: Option<usize>,
}

impl Default for PageBounds {
    fn default() -> Self {
        Self{
            count: None,
            offset: None,
        }
    }
}

/// Time bounded object supports limiting queries by time
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, StructOpt)]
pub struct TimeBounds {
    /// Start time for data query
    #[structopt(long, parse(try_from_str = timestamp_from_str))]
    pub from: Option<SystemTime>,

    /// End time for data query
    #[structopt(long, parse(try_from_str = timestamp_from_str))]
    pub until: Option<SystemTime>,
}

impl Default for TimeBounds {
    fn default() -> Self {
        Self{
            from: None,
            until: None,
        }
    }
}

/// Specific request kinds for issuing requests to the daemon from the client
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, StructOpt)]
pub enum RequestKind {
    #[structopt(name = "status")]
    /// Checks the status of the DSF daemon
    Status,

    #[structopt(name = "peer")]
    /// Subcommand for managing and interacting with peers
    Peer(PeerCommands),

    #[structopt(name = "service")]
    /// Subcommand for managing and interacting with services
    Service(ServiceCommands),

    #[structopt(name = "ns")]
    /// Subcommand for managing and interacting with name services
    Ns(NsCommands),

    /// Object requests
    #[structopt(name = "page")]
    Page(PageCommands),

    #[structopt(name = "data")]
    /// Subcommand for managing data
    Data(DataCommands),

    #[structopt(name = "subscriber")]
    /// Subcommand for managing subscribers
    Subscriber(SubscriberCommands),

    #[structopt(name = "config")]
    /// Subcommand for managing runtime daemon configuration
    Config(ConfigCommands),

    #[structopt(name = "debug")]
    /// Subcommand for exposing debug information
    Debug(DebugCommands),

    /// Stream data from a given service
    #[structopt(name = "stream")]
    Stream(SubscribeOptions),

}

/// Response container for replies from the daemon to the client
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Response {
    req_id: u64,
    kind: ResponseKind,
}

impl Response {
    pub fn new(req_id: u64, kind: ResponseKind) -> Self {
        Self { req_id, kind }
    }

    pub fn req_id(&self) -> u64 {
        self.req_id
    }

    pub fn kind(&self) -> ResponseKind {
        self.kind.clone()
    }
}

/// Specific response kinds for processing responses from the daemon
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ResponseKind {
    None,

    Status(StatusInfo),
    Connected(ConnectInfo),

    Peer(PeerInfo),
    Peers(Vec<(Id, PeerInfo)>),

    Service(ServiceInfo),
    Services(Vec<ServiceInfo>),
    Registered(RegisterInfo),
    Located(LocateInfo),

    Subscribed(Vec<SubscriptionInfo>),

    Published(PublishInfo),

    Datastore(Vec<(Id, Vec<Vec<u8>>)>),

    Ns(NsRegisterInfo),

    Data(Vec<DataInfo>),

    Pages(Vec<Container>),

    Page(Container),

    //Value(String),
    Unrecognised,

    Error(Error),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatusInfo {
    pub id: Id,
    pub peers: usize,
    pub services: usize,
}

pub use dsf_core::base::Body;

/// Parse a timestamp from a provided string
fn timestamp_from_str(s: &str) -> Result<SystemTime, chrono_english::DateError> {
    let t =
        chrono_english::parse_date_string(s, chrono::Local::now(), chrono_english::Dialect::Uk)?;
    Ok(t.into())
}
