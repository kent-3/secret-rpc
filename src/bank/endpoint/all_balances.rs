//! `/all_balances` endpoint JSON-RPC wrapper

use serde::{Deserialize, Serialize};

use crate::{bank::BankMethod, request::RequestMessage};
use cosmrs::{rpc::dialect::Dialect, Coin};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Request;

impl RequestMessage<BankMethod> for Request {
    fn method(&self) -> BankMethod {
        BankMethod::AllBalances
    }
}

impl<S: Dialect> crate::Request<S> for Request {
    type Response = Response;
}

impl<S: Dialect> crate::SimpleRequest<S> for Request {
    type Output = Response;
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Response {
    pub balances: Vec<Coin>,
    pub pagination: PageResponse,
}

impl crate::Response for Response {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PageResponse {
    pub next_key: Vec<u8>,
    pub total: u64,
}
