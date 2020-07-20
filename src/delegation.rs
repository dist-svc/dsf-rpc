use structopt::StructOpt;

use crate::ServiceIdentifier;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, StructOpt)]
pub enum DelegationCommands {
    #[structopt(name = "datastore")]
    /// Request a signed hello / discovery payload
    Hello,

    /// Register a service
    Register(Page),


}
