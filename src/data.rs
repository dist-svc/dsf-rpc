use structopt::StructOpt;

use dsf_core::base::Body;
use dsf_core::page::Page;
use dsf_core::types::*;

use crate::helpers::data_from_str;
use crate::{PageBounds, ServiceIdentifier};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataInfo {
    pub service: Id,

    pub index: u16,
    pub body: Body,

    pub previous: Option<Signature>,
    pub signature: Signature,
}

impl std::convert::TryFrom<&Page> for DataInfo {
    type Error = std::convert::Infallible;

    fn try_from(page: &Page) -> Result<DataInfo, Self::Error> {
        Ok(DataInfo {
            service: page.id.clone(),
            index: page.header().index,
            body: page.body().clone(),
            previous: page.previous_sig.clone(),
            signature: page.signature.clone().unwrap(),
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
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, StructOpt)]
pub struct ListOptions {
    #[structopt(flatten)]
    pub service: ServiceIdentifier,

    #[structopt(flatten)]
    pub bounds: PageBounds,
}

pub type Data = Vec<u8>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, StructOpt)]
pub struct PublishOptions {
    #[structopt(flatten)]
    pub service: ServiceIdentifier,

    #[structopt(short = "k", long = "kind", default_value = "generic")]
    /// Data page kind (defaults to generic)
    pub kind: DataKind,

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
            kind: DataKind::Generic,
            data: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PublishInfo {
    pub index: u16,
    //pub sig: Signature,
}
