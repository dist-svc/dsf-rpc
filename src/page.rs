
use structopt::StructOpt;
use serde::{Serialize, Deserialize};

use dsf_core::types::*;

use crate::ServiceIdentifier;


#[derive(Clone, Debug, PartialEq, StructOpt, Serialize, Deserialize)]
pub enum PageCommands {
    /// Fetch a page by signature
    Fetch(FetchOptions)
}

#[derive(Clone, Debug, PartialEq, StructOpt, Serialize, Deserialize)]
pub struct FetchOptions {
    /// Service identifier
    #[structopt(flatten)]
    pub service: ServiceIdentifier,

    /// Page signature
    #[structopt(long)]
    pub page_sig: Signature,
}

