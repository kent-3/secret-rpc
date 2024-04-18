//! JSON-RPC request methods

use core::{
    fmt::{self, Display},
    str::FromStr,
};

use serde::{de::Error as _, Deserialize, Deserializer, Serialize, Serializer};

use crate::prelude::*;
use cosmrs::rpc::Error;

/// JSON-RPC request methods.
///
/// Serialized as the "method" field of JSON-RPC/HTTP requests.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum BankMethod {
    Balance,
    AllBalances,
    TotalSupply,
}

impl crate::method::Meth for BankMethod {
    /// Get a static string which represents this method name
    fn as_str(self) -> &'static str {
        match self {
            BankMethod::Balance => "balance",
            BankMethod::AllBalances => "all_balance",
            BankMethod::TotalSupply => "total_supply",
        }
    }
}

impl FromStr for BankMethod {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        Ok(match s {
            "balance" => BankMethod::Balance,
            "all_balances" => BankMethod::AllBalances,
            "total_supply" => BankMethod::TotalSupply,
            other => return Err(Error::method_not_found(other.to_string())),
        })
    }
}

impl Display for BankMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Serialize for BankMethod {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.as_str().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for BankMethod {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::from_str(&String::deserialize(deserializer)?)
            .map_err(|e| D::Error::custom(format!("{e}")))
    }
}
