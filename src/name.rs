use std::time::SystemTime;

use structopt::StructOpt;
use serde::{Serialize, Deserialize};

use dsf_core::{prelude::*, types::CryptoHash};

use crate::ServiceIdentifier;


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, StructOpt)]
pub enum NsCommands {
    #[structopt()]
    /// Search using the specified name service
    Search(NsSearchOptions),
    
    #[structopt()]
    /// TODO: De-register an external address for the daemon
    Register(NsRegisterOptions),
}


#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, StructOpt)]
pub struct NsSearchOptions {
    #[structopt(flatten)]
    /// NameServer filter / selection
    pub ns: ServiceIdentifier,

    #[structopt(long, group="filters")]
    /// Service for search operation
    pub name: Option<String>,

    #[structopt(long, group="filters")]
    /// Hashes for searching
    pub hash: Option<CryptoHash>,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, StructOpt)]
pub struct NsRegisterOptions {
    #[structopt(flatten)]
    /// NameServer filter / selection
    pub ns: ServiceIdentifier,

    /// ID of service to register
    pub target: Id,

    #[structopt(long)]
    /// Service for registration
    pub name: Option<String>,

    #[structopt(long)]
    /// Hashes to associate with this service
    pub hash: Vec<CryptoHash>,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct NsRegisterInfo {
    pub ns: Id,
    pub prefix: Option<String>,

    pub name: Option<String>,
    pub hashes: Vec<CryptoHash>,
}
