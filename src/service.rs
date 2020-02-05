
use std::time::SystemTime;

use structopt::StructOpt;

use dsf_core::types::*;

use crate::{ServiceIdentifier, Body};
pub use crate::helpers::{try_load_file, try_parse_key_value};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
//#[cfg_attr(feature = "diesel", derive(diesel::Queryable))] 
//#[cfg_attr(feature = "diesel", table_name="services")]
pub struct ServiceInfo {
    pub id: Id,
    pub index: usize,
    pub state: ServiceState,

    pub public_key: PublicKey,
    pub secret_key: Option<SecretKey>,

    pub last_updated: Option<SystemTime>,
    
    pub primary_page: Option<Signature>,
    pub replica_page: Option<Signature>,

    pub subscribers: usize,
    pub replicas: usize,
    pub origin: bool,
}

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize, Display, EnumString)]
pub enum ServiceState {
    Created,
    Registered,
    Located,
    Subscribed,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, StructOpt)]
pub enum ServiceCommands {
    #[structopt(name = "list")]
    /// List known services
    List(ListOptions),

    #[structopt(name = "create")]
    /// Create a new service
    Create(CreateOptions),

    #[structopt(name = "register")]
    /// Create am existing service
    Register(RegisterOptions),

    #[structopt(name = "search")]
    /// Search for an existing service
    Search(LocateOptions),

    #[structopt(name = "subscribe")]
    /// Subscribe to a service
    Subscribe(SubscribeOptions),

    #[structopt(name = "unsubscribe")]
    /// Subscribe to a service
    Unsubscribe(UnsubscribeOptions),

    #[structopt(name = "set-key")]
    /// Set the encryption/decryption key for a given service
    SetKey(SetKeyOptions),
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, StructOpt)]
pub struct ListOptions {
    #[structopt(long)]
    /// Application ID for filtering
    pub application_id: Option<u16>,
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, StructOpt)]
pub struct CreateOptions {
    #[structopt(short = "i", long = "application-id", default_value = "0")]
    /// Application ID    
    pub application_id: u16,

    #[structopt(short = "k", long = "page-kind")]
    /// Page Kind (defaults to Generic)
    pub page_kind: Option<u16>,

    #[structopt(name = "body", parse(try_from_str = try_load_file))]
    /// Service Page Body (loaded from the specified file)
    pub body: Option<Body>,

    #[structopt(short = "a", long = "address")]
    /// Service Addresses
    pub addresses: Vec<Address>,

    #[structopt(short = "m", long = "metadata", parse(try_from_str = try_parse_key_value))]
    /// Service Metadata key:value pairs
    pub metadata: Vec<(String, String)>,

    #[structopt(short = "p", long = "public")]
    /// Indicate the service should be public (unencrypted)
    pub public: bool,

    #[structopt(long = "register")]
    /// Indicate the service should be registered and replicated following creation
    pub register: bool,
}

impl Default for CreateOptions {
    fn default() -> Self {
        Self{
            application_id: 0,
            page_kind: None,
            body: None,
            addresses: vec![],
            metadata: vec![],
            public: false,
            register: false,
        }
    }
}

impl CreateOptions {
    pub fn and_register(mut self) -> Self {
        self.register = true;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateInfo {
    pub id: Id,
    pub secret_key: Option<SecretKey>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, StructOpt)]
pub struct RegisterOptions {
    #[structopt(flatten)]
    pub service: ServiceIdentifier,

    #[structopt(long="no-replica")]
    /// Do not become a replica for the registered service
    pub no_replica: bool,
}

impl RegisterOptions {
    pub fn new(id: Id) -> Self {
        Self{service: ServiceIdentifier{id: Some(id), index: None}, no_replica: false}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegisterInfo {
    pub page_version: u16,
    pub replica_version: Option<u16>,
    pub peers: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, StructOpt)]
pub struct LocateOptions {
    #[structopt(short = "i", long = "id")]
    /// ID of the service to locate
    pub id: Id,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocateInfo {
    pub origin: bool,
    pub updated: bool,
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, StructOpt)]
pub struct SubscribeOptions {
    #[structopt(flatten)]
    pub service: ServiceIdentifier,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, StructOpt)]
pub struct UnsubscribeOptions {
    #[structopt(flatten)]
    pub service: ServiceIdentifier,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubscribeInfo {
    /// Number of replicas successfully connected
    pub count: u32
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, StructOpt)]
pub struct SetKeyOptions {
    #[structopt(flatten)]
    pub service: ServiceIdentifier,

    #[structopt(short = "s", long = "secret-key")]
    /// Secret key for service access
    pub secret_key: Option<SecretKey>,
}