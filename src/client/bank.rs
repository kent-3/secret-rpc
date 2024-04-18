use async_trait::async_trait;
use cosmrs::rpc::Error;

use crate::bank::endpoint::*;
use crate::bank::BankMethod;
use crate::SimpleRequest;

/// Provides lightweight access to the Bank RPC.
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait BankClient {
    /// `/balance`
    async fn balance(&self) -> Result<balance::Response, Error> {
        self.perform(balance::Request).await
    }

    /// `/all_balances`
    async fn all_balances(&self) -> Result<all_balances::Response, Error> {
        self.perform(all_balances::Request).await
    }

    /// Perform a request against the RPC endpoint.
    ///
    /// This method is used by the default implementations of specific
    /// endpoint methods. The latest protocol dialect is assumed to be invoked.
    async fn perform<R>(&self, request: R) -> Result<R::Output, Error>
    where
        R: SimpleRequest<BankMethod>;
}
