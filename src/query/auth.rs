// TODO - Add query methods (not sure if they exist... they are not in secretjs):
// ModuleAccounts, AccountAddressByID, AddressStringToBytes, AddressBytesToString, Bech32Prefix

use ::cosmrs::proto::cosmos::auth::v1beta1::*;
use ::cosmrs::proto::cosmos::base::query::v1beta1::PageRequest;
use ::cosmrs::proto::cosmos::vesting::v1beta1::{ContinuousVestingAccount, DelayedVestingAccount};

use super::{try_decode_any, try_decode_response};
use crate::{Error, Result};

#[derive(Debug)]
pub enum Account {
    BaseAccount(::cosmrs::auth::BaseAccount),
    ModuleAccount(::cosmrs::auth::ModuleAccount),
    ContinuousVestingAccount(::cosmrs::vesting::ContinuousVestingAccount),
    DelayedVestingAccount(::cosmrs::vesting::DelayedVestingAccount),
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
                let account: BaseAccount = any.to_msg()?;
                Ok(Account::BaseAccount(account.try_into()?))
            }
            "/cosmos.auth.v1beta1.ModuleAccount" => {
                let account: ModuleAccount = any.to_msg()?;
                Ok(Account::ModuleAccount(account.try_into()?))
            }
            // trait 'Name' is not implemented for these
            "/cosmos.vesting.v1beta1.ContinuousVestingAccount" => {
                let account = try_decode_any::<ContinuousVestingAccount>(any)?;
                Ok(Account::ContinuousVestingAccount(account.try_into()?))
            }
            "/cosmos.vesting.v1beta1.DelayedVestingAccount" => {
                let account = try_decode_any::<DelayedVestingAccount>(any)?;
                Ok(Account::DelayedVestingAccount(account.try_into()?))
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
            .and_then(|x| x.params.ok_or(Error::AbciQuery("empty params".to_string())))
    }

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
                res.account
                    .ok_or(Error::AbciQuery("empty account".to_string()))
            })
            .and_then(try_decode_any::<ModuleAccount>)
    }
}
