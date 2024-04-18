#![allow(unused)]

use cosmrs::proto::cosmos::auth::v1beta1::*;
use cosmrs::proto::cosmos::base::query::v1beta1::PageRequest;
use cosmrs::proto::cosmos::vesting::v1beta1::{ContinuousVestingAccount, DelayedVestingAccount};
use cosmrs::rpc::endpoint::abci_query::AbciQuery as QueryResponse;
use cosmrs::Any;

use prost::Message;

use super::{try_decode_any, try_decode_response};
use crate::{Error, Result};

#[derive(Debug)]
pub enum Account {
    BaseAccount(BaseAccount),
    ModuleAccount(ModuleAccount),
    ContinuousVestingAccount(ContinuousVestingAccount),
    DelayedVestingAccount(DelayedVestingAccount),
}

impl crate::Client {
    pub async fn auth_account(&self, address: impl Into<String>) -> Result<Account> {
        let address = address.into();

        let path = "/cosmos.auth.v1beta1.Query/Account";
        let msg = QueryAccountRequest { address };

        let any = self
            .query_with_msg(path, msg)
            .await
            .and_then(try_decode_response::<QueryAccountResponse>)
            .and_then(|res| {
                res.account
                    .ok_or(Error::AbciQuery("no account".to_string()))
            })?;

        match any.type_url.as_str() {
            "/cosmos.auth.v1beta1.BaseAccount" => {
                let account = try_decode_any::<BaseAccount>(any)?;
                Ok(Account::BaseAccount(account))
            }
            "/cosmos.auth.v1beta1.ModuleAccount" => {
                let account = ModuleAccount::decode(any.value.as_slice())?;
                Ok(Account::ModuleAccount(account))
            }
            "/cosmos.vesting.v1beta1.ContinuousVestingAccount" => {
                let account = ContinuousVestingAccount::decode(any.value.as_slice())?;
                Ok(Account::ContinuousVestingAccount(account))
            }
            "/cosmos.vesting.v1beta1.DelayedVestingAccount" => {
                let account = DelayedVestingAccount::decode(any.value.as_slice())?;
                Ok(Account::DelayedVestingAccount(account))
            }
            _ => Err(Error::AbciQuery(format!(
                "unexpected type_url: {}",
                any.type_url
            ))),
        }
    }

    // TODO - more work needed here to handle the response and pagination...
    pub async fn auth_accounts(
        &self,
        pagination: Option<PageRequest>,
    ) -> Result<QueryAccountsResponse> {
        let path = "/cosmos.auth.v1beta1.Query/Accounts";
        let msg = QueryAccountsRequest { pagination };

        self.query_with_msg(path, msg)
            .await
            .and_then(try_decode_response::<QueryAccountsResponse>)
    }

    pub async fn auth_params(&self) -> Result<Params> {
        let path = "/cosmos.auth.v1beta1.Query/Params";
        let msg = QueryParamsRequest {};

        self.query_with_msg(path, msg)
            .await
            .and_then(try_decode_response::<QueryParamsResponse>)
            .and_then(|x| {
                x.params
                    .ok_or_else(|| Error::AbciQuery("empty params".to_string()))
            })
    }

    // TODO - figure out which account types are possible to return here, to decode 'Any'
    pub async fn auth_module_account_by_name(
        &self,
        name: impl Into<String>,
    ) -> Result<ModuleAccount> {
        let name = name.into();

        let path = "/cosmos.auth.v1beta1.Query/ModuleAccountByName";
        let msg = QueryModuleAccountByNameRequest { name: name.clone() };

        self.query_with_msg(path, msg)
            .await
            .and_then(try_decode_response::<QueryModuleAccountByNameResponse>)
            .and_then(|res| {
                res.account.ok_or(Error::AbciQuery(format!(
                    "module account \"{}\" not found",
                    name
                )))
            })
            .and_then(try_decode_any::<ModuleAccount>)
    }
}
