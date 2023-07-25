use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::Addr;

use super::query_response::QueryResponseType;

#[non_exhaustive]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DistributionQuery {
    /// See <https://github.com/cosmos/cosmos-sdk/blob/c74e2887b0b73e81d48c2f33e6b1020090089ee0/proto/cosmos/distribution/v1beta1/query.proto#L222-L230>
    DelegatorWithdrawAddress { delegator_address: String },
    /// See <https://github.com/cosmos/cosmos-sdk/blob/c74e2887b0b73e81d48c2f33e6b1020090089ee0/proto/cosmos/distribution/v1beta1/query.proto#L157-L167>
    #[cfg(feature = "cosmwasm_1_4")]
    DelegationRewards {
        delegator_address: String,
        validator_address: String,
    },
    /// See <https://github.com/cosmos/cosmos-sdk/blob/c74e2887b0b73e81d48c2f33e6b1020090089ee0/proto/cosmos/distribution/v1beta1/query.proto#L180-L187>
    #[cfg(feature = "cosmwasm_1_4")]
    DelegationTotalRewards { delegator_address: String },
}

/// See <https://github.com/cosmos/cosmos-sdk/blob/c74e2887b0b73e81d48c2f33e6b1020090089ee0/proto/cosmos/distribution/v1beta1/query.proto#L232-L240>
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct DelegatorWithdrawAddressResponse {
    pub withdraw_address: Addr,
}

impl_response_constructor!(DelegatorWithdrawAddressResponse, withdraw_address: Addr);

impl QueryResponseType for DelegatorWithdrawAddressResponse {}

/// See <https://github.com/cosmos/cosmos-sdk/blob/c74e2887b0b73e81d48c2f33e6b1020090089ee0/proto/cosmos/distribution/v1beta1/query.proto#L169-L178>
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct DelegationRewardsResponse {
    pub rewards: Vec<DecCoin>,
}

impl_response_constructor!(DelegationRewardsResponse, rewards: Vec<DecCoin>);
impl QueryResponseType for DelegationRewardsResponse {}

/// A coin type with decimal amount.
/// Modeled after the Cosmos SDK's [DecCoin](https://github.com/cosmos/cosmos-sdk/blob/c74e2887b0b73e81d48c2f33e6b1020090089ee0/proto/cosmos/base/v1beta1/coin.proto#L32-L41) type
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct DecCoin {
    pub denom: String,
    pub amount: crate::Decimal,
}

/// See <https://github.com/cosmos/cosmos-sdk/blob/c74e2887b0b73e81d48c2f33e6b1020090089ee0/proto/cosmos/distribution/v1beta1/query.proto#L189-L200>
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[non_exhaustive]
pub struct DelegationTotalRewardsResponse {
    pub rewards: Vec<DelegatorReward>,
    pub total: Vec<DecCoin>,
}

impl_response_constructor!(
    DelegationTotalRewardsResponse,
    rewards: Vec<DelegatorReward>,
    total: Vec<DecCoin>
);
impl QueryResponseType for DelegationTotalRewardsResponse {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct DelegatorReward {
    pub validator_address: String,
    pub reward: Vec<DecCoin>,
}
