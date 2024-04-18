//! JSON-RPC request methods

use core::{
    fmt::{self, Display},
    str::FromStr,
};

use serde::{de::Error as _, Deserialize, Deserializer, Serialize, Serializer};

use crate::prelude::*;
use cosmrs::rpc::Error;

use crate::bank::BankMethod;

/// JSON-RPC request methods.
///
/// Serialized as the "method" field of JSON-RPC/HTTP requests.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum Method {
    // bank module
    Banking(crate::bank::BankMethod),
}

pub trait Meth {
    fn as_str(self) -> &'static str;
}

impl Meth for Method {
    /// Get a static string which represents this method name
    fn as_str(self) -> &'static str {
        match self {
            // bank module
            Method::Banking(BankMethod::Balance) => "balance",
            Method::Banking(BankMethod::AllBalances) => "all_balance",
            Method::Banking(BankMethod::TotalSupply) => "total_supply",
        }
    }
}

impl FromStr for Method {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        Ok(match s {
            "balance" => Method::Banking(BankMethod::Balance),
            "all_balances" => Method::Banking(BankMethod::AllBalances),
            "total_supply" => Method::Banking(BankMethod::TotalSupply),
            other => return Err(Error::method_not_found(other.to_string())),
        })
    }
}

impl Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Serialize for Method {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.as_str().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Method {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::from_str(&String::deserialize(deserializer)?)
            .map_err(|e| D::Error::custom(format!("{e}")))
    }
}
