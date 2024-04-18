extern crate alloc;
extern crate std;

// Copied from tendermint-rpc
mod prelude;

// pub mod dialect;
// pub mod endpoint;
// pub mod error;
// pub mod event;
mod id;
mod method;
// mod order;
// mod paging;
// pub mod query;
pub mod request;
pub mod response;
pub mod response_error;
// mod rpc_url;
// pub mod serializers;
mod utils;
mod version;

// pub use error::Error;
pub use id::Id;
pub use method::Method;
// pub use order::Order;
// pub use paging::{PageNumber, Paging, PerPage};
pub use request::{Request, SimpleRequest};
pub use response::Response;
pub use response_error::{Code, ResponseError};
// pub use rpc_url::{Scheme, Url};
pub use version::Version;

use crate::consts::{DEFAULT_PORT, TESTNET_CHAIN_ID, TESTNET_ENCLAVE_KEY, TESTNET_HOST};
use crate::error::Result;

pub mod account;
pub mod client;
pub(crate) mod consts;
pub(crate) mod crypto;
pub mod error;

pub use account::{a, b, c, d, Account};
pub use client::{
    tx::builder::*,
    types::{CodeHash, CodeId, Contract, TxResponse},
    Client,
};
pub use error::Error;

// Cosmos SDK
pub mod bank;

// Re-exports
pub use cosmrs::rpc::Client as TendermintClient;

pub struct SecretRPC {
    /// RPC server URL
    host: String,
    port: u16,
    /// Hex-encoded Enclave Public Key
    enclave_key: String,
    chain_id: String,
}

impl SecretRPC {
    /// Initializes the constructor as a testnet instance
    pub fn new() -> Self {
        Self {
            host: TESTNET_HOST.to_owned(),
            port: DEFAULT_PORT,
            enclave_key: TESTNET_ENCLAVE_KEY.to_owned(),
            chain_id: TESTNET_CHAIN_ID.to_owned(),
        }
    }

    pub fn host(mut self, host: impl Into<String>) -> Self {
        self.host = host.into();
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    pub fn enclave_key(mut self, key: impl Into<String>) -> Self {
        self.enclave_key = key.into();
        self
    }

    pub fn chain_id(mut self, id: impl Into<String>) -> Self {
        self.chain_id = id.into();
        self
    }

    pub fn connect(&self) -> Result<Client> {
        let enclave_key = crypto::clone_into_key(&hex::decode(&self.enclave_key)?);

        Client::init(&self.host, self.port, enclave_key, &self.chain_id)
    }
}
