use std::net::SocketAddr;
use structopt::StructOpt;

use crate::helpers::try_parse_sock_addr;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, StructOpt)]
pub enum ConfigCommands {
    #[structopt(name = "add-address")]
    /// TODO: Register an external address for use by the daemon
    AddAddress(SocketAddress),

    #[structopt(name = "remove-address")]
    /// TODO: De-register an external address for the daemon
    RemoveAddress(SocketAddress),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, StructOpt)]
pub struct SocketAddress {
    #[structopt(parse(try_from_str = try_parse_sock_addr))]
    /// Peer socket address
    pub address: SocketAddr,
}
