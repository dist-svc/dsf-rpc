
use structopt::StructOpt;

use dsf_core::types::*;
use dsf_core::base::Body;

use crate::{ServiceIdentifier};
use crate::helpers::{data_from_str};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataInfo {
    pub service: Id,

    pub index: u16,
    pub body: Body,
    pub parent: Option<Id>,

    pub signature: Signature,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, StructOpt)]
pub enum DataCommands {
    #[structopt(name = "list")]
    /// List service data
    List(ListOptions),

    #[structopt(name = "sync")]
    /// Synchronize service data
    Update{},
        
    #[structopt(name = "query")]
    /// Fetch data from a service
    Query{},

    #[structopt(name = "publish")]
    /// Publish data to a service
    Publish(PublishOptions),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, StructOpt)]
pub struct ListOptions {
    #[structopt(flatten)]
    pub service: ServiceIdentifier,

    #[structopt(short = "c", long="count", default_value="1")]
    pub n: usize,
}

pub type Data = Vec<u8>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, StructOpt)]
pub struct PublishOptions {
    #[structopt(flatten)]
    pub service: ServiceIdentifier,

    #[structopt(short = "k", long = "kind")]
    /// Data page kind (defaults to generic)
    pub kind: Option<DataKind>,

    #[structopt(short = "d", long = "data", parse(from_str = data_from_str))]
    /// Data body as a string
    pub data: Option<Data>,
}


impl PublishOptions {
    pub fn new(id: Id) -> Self {
        Self{service: ServiceIdentifier{id: Some(id), index: None}, kind: None, data: None}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PublishInfo {
    pub index: u16,
    //pub sig: Signature,
}
