
use std::net::SocketAddr;
use std::str::FromStr;
use std::time::Duration;

extern crate chrono_humanize;

extern crate humantime;
use humantime::Duration as HumanDuration;

extern crate futures;
use futures::prelude::*;
extern crate structopt;
use structopt::StructOpt;

extern crate rand;
use rand::random;

extern crate actix;
use actix::Message;

extern crate dsf_core;
use dsf_core::types::*;

#[macro_use]
extern crate serde;

extern crate colored;
use colored::Colorize;

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

pub mod display;

/// API trait implements RPC API for the daemon (or delegation)
pub trait Rpc {
    type Error;

    fn exec(&mut self, req: Request) -> Box<dyn Future<Item=Response, Error=Self::Error>>;
}

/// RPC Request container for requests from a client to the daemon
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Request {
    req_id: u64,
    kind: RequestKind,
}

impl Message for Request {
    type Result = Result<Response, Error>;
}

impl Request {
    pub fn new(kind: RequestKind) -> Self {
        Self{
            req_id: random(),
            kind
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
    #[structopt(short = "i", long = "id")]
    /// Global service ID
    pub id: Option<Id>,

    #[structopt(short = "n", long = "index")]
    /// Local service index
    pub index: Option<usize>,
}

impl ServiceIdentifier {
    pub fn id(id: Id) -> Self {
        Self{id: Some(id), index: None}
    }

    pub fn index(index: usize) -> Self {
        Self{id: None, index: Some(index)}
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

    #[structopt(name = "data")]
    /// Subcommand for managing data
    Data(DataCommands),

    #[structopt(name = "config")]
    /// Subcommand for managing runtime daemon configuration
    Config(ConfigCommands),

    #[structopt(name = "debug")]
    /// Subcommand for exposing debug information
    Debug(DebugCommands),

    /// Stream data from a given service
    #[structopt(name = "stream")]
    Stream(StreamOptions),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, StructOpt)]
pub struct StreamOptions {
    #[structopt(flatten)]
    pub service: ServiceIdentifier,
}

/// Response container for replies from the daemon to the client
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Message)]
pub struct Response {
    req_id: u64,
    kind: ResponseKind,
}

impl Response {
    pub fn new(req_id: u64, kind: ResponseKind) -> Self {
        Self{
            req_id,
            kind,
        }
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
    Peers(Vec<(Id, PeerInfo)>),

    Created(ServiceInfo),
    Services(Vec<ServiceInfo>),
    Registered(RegisterInfo),
    Located(LocateInfo),
    Subscribed(SubscribeInfo),
    
    Published(PublishInfo),

    Datastore(Vec<(Id, Vec<Vec<u8>>)>),

    Data(Vec<DataInfo>),

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




use std::net::{ToSocketAddrs};
use std::io::{Error as IoError, ErrorKind as IoErrorKind};


pub fn try_parse_sock_addr(from: &str) -> Result<SocketAddr, IoError> {
    let mut addrs = from.to_socket_addrs()?;

    match addrs.next() {
        Some(a) => Ok(a),
        None => Err(IoError::new(IoErrorKind::Other, "no socket addresses found")),
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body {
    pub data: Vec<u8>,
}

use std::fs;
pub fn try_load_file(from: &str) -> Result<Body, IoError> {
    let data = fs::read(from)?;
    Ok(Body{data})
}

use std::str;

pub fn try_parse_kv(from: &str) -> Result<(String, String), IoError> {
    let split: Vec<_> = from.split(":").collect();
    if split.len() != 2 {
        return Err(IoError::new(IoErrorKind::Other, "key:value pair parsing failed"));
    }

    Ok((split[0].to_owned(), split[1].to_owned()))
}


fn parse_duration(s: &str) -> Result<Duration, humantime::DurationError> {
    let hd = HumanDuration::from_str(s)?;
    Ok(hd.into())
}