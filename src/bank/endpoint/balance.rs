//! `/abci_info` endpoint JSON-RPC wrapper

use serde::{Deserialize, Serialize};

use crate::{bank::BankMethod, request::RequestMessage};
use cosmrs::{rpc::dialect::Dialect, Coin};

/// Request ABCI information from a node
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Request;

impl RequestMessage<BankMethod> for Request {
    fn method(&self) -> BankMethod {
        BankMethod::Balance
    }
}

impl<S: Dialect> crate::Request<S> for Request {
    type Response = Response;
}

impl<S: Dialect> crate::SimpleRequest<S> for Request {
    type Output = Response;
}

/// Bank balance response
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Response {
    /// Bank balance
    pub balance: Coin,
}

impl crate::Response for Response {}
