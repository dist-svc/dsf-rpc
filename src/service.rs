use std::net::SocketAddr;
use std::time::SystemTime;

use strum::{VariantNames};

use dsf_core::{wire::Container, options::Options, prelude::Service};
use structopt::StructOpt;

use dsf_core::types::*;

pub use crate::helpers::{try_load_file, try_parse_key_value};
use crate::{Body, ServiceIdentifier};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
//#[cfg_attr(feature = "diesel", derive(diesel::Queryable))]
//#[cfg_attr(feature = "diesel", table_name="services")]
pub struct ServiceInfo {
    pub id: Id,
    pub index: usize,
    pub state: ServiceState,

    pub public_key: PublicKey,
    pub private_key: Option<PrivateKey>,
    pub secret_key: Option<SecretKey>,

    pub last_updated: Option<SystemTime>,

    pub primary_page: Option<Signature>,
    pub replica_page: Option<Signature>,

    pub subscribers: usize,
    pub replicas: usize,
    pub origin: bool,
    pub subscribed: bool,
}

impl From<&Service> for ServiceInfo {
    /// Create a default service info object for a service.
    /// Note that fields undefined within the service will be zero-initialised
    fn from(svc: &Service) -> Self {
        Self{
            id: svc.id(),
            index: svc.version() as usize,
            state: ServiceState::Created,
            public_key: svc.public_key(),
            private_key: svc.private_key(),
            secret_key: svc.secret_key(),

            last_updated: None,
            primary_page: None,
            replica_page: None,
            
            subscribers: 0,
            replicas: 0,
            origin: svc.is_origin(),
            subscribed: false,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize, Display)]
#[cfg_attr(feature = "std", derive(EnumString))]
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

    #[structopt(name = "locate")]
    /// Locate an existing service
    Locate(LocateOptions),

    #[structopt(name = "info")]
    /// Fetch information for a service
    Info(InfoOptions),

    #[structopt(name = "register")]
    /// Register an existing / known service
    Register(RegisterOptions),

    #[structopt(name = "subscribe")]
    /// Subscribe to a known service
    Subscribe(SubscribeOptions),

    #[structopt(name = "unsubscribe")]
    /// Unsubscribe from a known service
    Unsubscribe(UnsubscribeOptions),

    #[structopt(name = "set-key")]
    /// Set the encryption/decryption key for a given service
    SetKey(SetKeyOptions),

    #[structopt(name = "remove")]
    /// Remove a service from the service list (and database if specified)
    Remove(RemoveOptions),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, StructOpt)]
pub struct ListOptions {
    #[structopt(long = "application-id")]
    /// Application ID for filtering
    pub application_id: Option<u16>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, StructOpt)]
pub struct CreateOptions {
    #[structopt(short = "i", long = "application-id", default_value = "0")]
    /// Application ID    
    pub application_id: u16,

    #[structopt(short = "k", long = "page-kind", possible_values=PageKind::VARIANTS)]
    /// Page Kind (defaults to Generic)
    pub page_kind: Option<PageKind>,

    #[structopt(name = "body", parse(try_from_str = try_load_file))]
    /// Service Page Body (loaded from the specified file)
    pub body: Option<Data>,

    #[structopt(short = "a", long = "address")]
    /// Service Addresses
    pub addresses: Vec<SocketAddr>,

    #[structopt(short = "m", long = "metadata", parse(try_from_str = try_parse_key_value))]
    /// Service Metadata key:value pairs
    pub metadata: Vec<(String, String)>,

    #[structopt(long)]
    /// Service metadata / options
    pub public_options: Vec<Options>,

    #[structopt(long)]
    /// Service metadata / options
    pub private_options: Vec<Options>,

    #[structopt(short = "p", long = "public")]
    /// Indicate the service should be public (unencrypted)
    pub public: bool,

    #[structopt(long = "register")]
    /// Indicate the service should be registered and replicated following creation
    pub register: bool,
}

pub type Data = Vec<u8>;

impl Default for CreateOptions {
    fn default() -> Self {
        Self {
            application_id: 0,
            page_kind: None,
            body: None,
            addresses: vec![],
            metadata: vec![],
            public_options: vec![],
            private_options: vec![],
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

    #[structopt(long = "no-replica")]
    /// Do not become a replica for the registered service
    pub no_replica: bool,
}

impl RegisterOptions {
    pub fn new(id: Id) -> Self {
        Self {
            service: ServiceIdentifier {
                id: Some(id),
                index: None,
            },
            no_replica: false,
        }
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

    #[structopt(long = "local-only")]
    /// Search only in the local datastore
    pub local_only: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocateInfo {
    pub origin: bool,
    pub updated: bool,
    pub page_version: u16,
    #[serde(skip)]
    pub page: Option<Container>,
}

impl Default for LocateInfo {
    fn default() -> Self {
        Self{
            origin: false,
            updated: false,
            page_version: 0,
            page: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, StructOpt)]
pub struct InfoOptions {
    #[structopt(flatten)]
    pub service: ServiceIdentifier,
}

impl From<ServiceIdentifier> for InfoOptions {
    fn from(service: ServiceIdentifier) -> Self {
        Self { service }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, StructOpt)]
pub struct SubscribeOptions {
    #[structopt(flatten)]
    pub service: ServiceIdentifier,
}

impl From<ServiceIdentifier> for SubscribeOptions {
    fn from(service: ServiceIdentifier) -> Self {
        Self { service }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, StructOpt)]
pub struct UnsubscribeOptions {
    #[structopt(flatten)]
    pub service: ServiceIdentifier,
}

impl From<ServiceIdentifier> for UnsubscribeOptions {
    fn from(service: ServiceIdentifier) -> Self {
        Self { service }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SubscriptionKind {
    Peer(Id),
    Socket(u32),
    None,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubscriptionInfo {
    pub service_id: Id,
    pub kind: SubscriptionKind,

    pub updated: Option<SystemTime>,
    pub expiry: Option<SystemTime>,

    pub qos: QosPriority,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum QosPriority {
    None = 0,
    Latency = 1,
}

impl SubscriptionInfo {
    pub fn new(service_id: Id, kind: SubscriptionKind) -> Self {
        Self{ service_id, kind, updated: None, expiry: None, qos: QosPriority::None }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, StructOpt)]
pub struct SetKeyOptions {
    #[structopt(flatten)]
    pub service: ServiceIdentifier,

    #[structopt(short = "s", long = "secret-key")]
    /// Secret key for service access
    pub secret_key: Option<SecretKey>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, StructOpt)]
pub struct RemoveOptions {
    #[structopt(flatten)]
    pub service: ServiceIdentifier,

    #[structopt(long)]
    /// Attempt to remove an owned service from the network
    pub purge: bool,
}
