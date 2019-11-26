
use structopt::StructOpt;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, StructOpt)]
pub enum DebugCommands {
    #[structopt(name = "datastore")]
    /// Subscribe to a service
    Datastore,

    #[structopt(name = "update")]
    /// Force an update of the daemon
    Update,

    #[structopt(name = "bootstrap")]
    /// Invoke bootstrapping
    Bootstrap,
}
