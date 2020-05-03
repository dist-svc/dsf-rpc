use structopt::StructOpt;

use crate::ServiceIdentifier;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, StructOpt)]
pub enum DebugCommands {
    #[structopt(name = "datastore")]
    /// Datastore debug commands
    Datastore(DatastoreCommands),

    #[structopt(name = "dht")]
    /// Datastore debug commands
    Dht(DhtCommands),

    #[structopt(name = "update")]
    /// Force an update of the daemon
    Update,

    #[structopt(name = "bootstrap")]
    /// Invoke bootstrapping
    Bootstrap,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, StructOpt)]
pub enum DatastoreCommands {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, StructOpt)]
pub enum DhtCommands {
    #[structopt(name = "peer")]
    /// Find a peer with a given ID
    Peer(ServiceIdentifier),

    #[structopt(name = "data")]
    /// Find data at a certain ID
    Data(ServiceIdentifier),
}
