use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::Coin;

#[cfg(feature = "cosmwasm_1_3")]
use crate::PageRequest;
use crate::{Binary, DenomMetadata};

use super::query_response::QueryResponseType;
///Defines the types of queries that can be made to the bank module.
///This includes queries for account balances, transaction history, etc.
#[non_exhaustive]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum BankQuery {
    /// This calls into the native bank module for querying the total supply of one denomination.
    /// It does the same as the SupplyOf call in Cosmos SDK's RPC API.
    /// Return value is of type SupplyResponse.
    #[cfg(feature = "cosmwasm_1_1")]
    Supply { denom: String },
    /// This calls into the native bank module for one denomination
    /// Return value is BalanceResponse
    Balance { address: String, denom: String },
    /// This calls into the native bank module for all denominations.
    /// Note that this may be much more expensive than Balance and should be avoided if possible.
    /// Return value is AllBalanceResponse.
    AllBalances { address: String },
    /// This calls into the native bank module for querying metadata for a specific bank token.
    /// Return value is DenomMetadataResponse
    #[cfg(feature = "cosmwasm_1_3")]
    DenomMetadata { denom: String },
    /// This calls into the native bank module for querying metadata for all bank tokens that have a metadata entry.
    /// Return value is AllDenomMetadataResponse
    #[cfg(feature = "cosmwasm_1_3")]
    AllDenomMetadata { pagination: Option<PageRequest> },
}
///A response format that provides information about the supply of a specific asset or token.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct SupplyResponse {
    /// Always returns a Coin with the requested denom.
    /// This will be of zero amount if the denom does not exist.
    pub amount: Coin,
}

impl_response_constructor!(SupplyResponse, amount: Coin);

impl QueryResponseType for SupplyResponse {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
///This struct holds the response information for a balance query, detailing the amounts held.
#[non_exhaustive]
pub struct BalanceResponse {
    /// Always returns a Coin with the requested denom.
    /// This may be of 0 amount if no such funds.
    pub amount: Coin,
}

impl_response_constructor!(BalanceResponse, amount: Coin);

impl QueryResponseType for BalanceResponse {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
///A response structure that contains all balances for a query,
///typically used in balance-related requests.
#[non_exhaustive]
pub struct AllBalanceResponse {
    /// Returns all non-zero coins held by this account.
    pub amount: Vec<Coin>,
}

impl_response_constructor!(AllBalanceResponse, amount: Vec<Coin>);

impl QueryResponseType for AllBalanceResponse {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
/// A response format that contains metadata for a specific denomination in a query.
pub struct DenomMetadataResponse {
    /// The metadata for the queried denom.
    pub metadata: DenomMetadata,
}

impl_response_constructor!(DenomMetadataResponse, metadata: DenomMetadata);

impl QueryResponseType for DenomMetadataResponse {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct AllDenomMetadataResponse {
    /// Always returns metadata for all token denoms on the base chain.
    pub metadata: Vec<DenomMetadata>,
    pub next_key: Option<Binary>,
}

impl_response_constructor!(
    AllDenomMetadataResponse,
    metadata: Vec<DenomMetadata>,
    next_key: Option<Binary>
);

impl QueryResponseType for AllDenomMetadataResponse {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn private_constructor_works() {
        let response = AllBalanceResponse::new(vec![Coin::new(1234u128, "uatom")]);
        assert_eq!(
            response,
            AllBalanceResponse {
                amount: vec![Coin::new(1234u128, "uatom")]
            }
        );
    }
}
