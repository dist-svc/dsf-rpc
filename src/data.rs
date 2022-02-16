use dsf_core::options::Filters;
use dsf_core::prelude::MaybeEncrypted;
use dsf_core::wire::Container;
use structopt::StructOpt;

use dsf_core::base::Body;
use dsf_core::types::*;

use crate::helpers::data_from_str;
use crate::{PageBounds, TimeBounds, ServiceIdentifier};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataInfo {
    pub service: Id,

    pub index: u16,
    pub body: Body,

    pub previous: Option<Signature>,
    pub signature: Signature,
}

impl std::convert::TryFrom<&Container> for DataInfo {
    type Error = std::convert::Infallible;

    fn try_from(page: &Container) -> Result<DataInfo, Self::Error> {
        let body = match page.encrypted() {
            true => MaybeEncrypted::Encrypted(page.body_raw().to_vec()),
            false => MaybeEncrypted::Cleartext(page.body_raw().to_vec()),
        };

        Ok(DataInfo {
            service: page.id(),
            index: page.header().index(),
            body,
            previous: page.public_options_iter().prev_sig(),
            signature: page.signature(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, StructOpt)]
pub enum DataCommands {
    #[structopt(name = "list")]
    /// List service data
    List(ListOptions),

    #[structopt(name = "sync")]
    /// Synchronize service data
    Update {},

    #[structopt(name = "query")]
    /// Fetch data from a service
    Query {},

    #[structopt(name = "publish")]
    /// Publish data to a service
    Publish(PublishOptions),

    #[structopt(name = "push")]
    /// Push pre-signed data for a known server
    Push(PushOptions),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, StructOpt)]
pub struct ListOptions {
    #[structopt(flatten)]
    pub service: ServiceIdentifier,

    #[structopt(flatten)]
    #[serde(default)]
    pub page_bounds: PageBounds,

    #[structopt(flatten)]
    #[serde(default)]
    pub time_bounds: TimeBounds,
}

pub type Data = Vec<u8>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, StructOpt)]
pub struct PublishOptions {
    #[structopt(flatten)]
    pub service: ServiceIdentifier,

    #[structopt(short = "k", long = "kind", default_value = "0")]
    /// Data page kind (defaults to generic)
    pub kind: u16,

    #[structopt(short = "d", long = "data", parse(from_str = data_from_str))]
    /// Data body as a string
    pub data: Option<Data>,
}

impl PublishOptions {
    pub fn new(id: Id) -> Self {
        Self {
            service: ServiceIdentifier {
                id: Some(id),
                index: None,
            },
            kind: 0,
            data: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, StructOpt)]
pub struct PushOptions {
    #[structopt(flatten)]
    pub service: ServiceIdentifier,

    #[structopt(short = "d", long = "data", parse(from_str = data_from_str))]
    /// Base64 encoded (pre-signed) DSF object
    pub data: Data,
}

impl PushOptions {
    pub fn new(id: Id, data: Vec<u8>) -> Self {
        Self {
            service: ServiceIdentifier {
                id: Some(id),
                index: None,
            },
            data,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PublishInfo {
    pub index: u16,
    //pub sig: Signature,
}
