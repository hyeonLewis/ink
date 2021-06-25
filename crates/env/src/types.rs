// Copyright 2018-2021 Parity Technologies (UK) Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Types for the default environment.
//!
//! These are simple mirrored types from the default substrate FRAME configuration.
//! Their interfaces and functionality might not be complete.
//!
//! Users are required to provide their own type definitions and `Environment`
//! implementations in order to write ink! contracts for other chain configurations.
//!
//! # Note
//!
//! When authoring a contract, the concrete `Environment` are available via aliases
//! generated by the `lang` macro. Therefore all functionality of the concrete
//! types is accessible in the contract, not constrained by the required trait
//! bounds.
//!
//! Outside the contract and its tests (e.g. in the off-chain environment), where
//! there is no knowledge of the concrete types, the functionality is restricted to
//! the trait bounds on the `Environment` trait types.

use super::arithmetic::AtLeast32BitUnsigned;
use core::{
    array::TryFromSliceError,
    convert::TryFrom,
};
use derive_more::From;
use scale::{
    Decode,
    Encode,
};
#[cfg(feature = "std")]
use scale_info::TypeInfo;
use sp_arithmetic::PerThing;
pub use sp_arithmetic::Perbill;

/// The environmental types usable by contracts defined with ink!.
pub trait Environment {
    /// The maximum number of supported event topics provided by the runtime.
    ///
    /// The value must match the maximum number of supported event topics of the used runtime.
    const MAX_EVENT_TOPICS: usize;

    /// The address type.
    type AccountId: 'static + scale::Codec + Clone + PartialEq + Eq + Ord;

    /// The type of balances.
    type Balance: 'static
        + scale::Codec
        + Copy
        + Clone
        + PartialEq
        + Eq
        + AtLeast32BitUnsigned;

    /// The type of hash.
    type Hash: 'static
        + scale::Codec
        + Copy
        + Clone
        + Clear
        + PartialEq
        + Eq
        + Ord
        + AsRef<[u8]>
        + AsMut<[u8]>;

    /// The type of timestamps.
    type Timestamp: 'static
        + scale::Codec
        + Copy
        + Clone
        + PartialEq
        + Eq
        + AtLeast32BitUnsigned;

    /// The type of block number.
    type BlockNumber: 'static
        + scale::Codec
        + Copy
        + Clone
        + PartialEq
        + Eq
        + AtLeast32BitUnsigned;

    /// The chain extension for the environment.
    ///
    /// This is a type that is defined through the `#[ink::chain_extension]` procedural macro.
    /// For more information about usage and definition click [this][chain_extension] link.
    ///
    /// [chain_extension]: https://paritytech.github.io/ink/ink_lang/attr.chain_extension.html
    type ChainExtension;

    /// The fraction of the deposit costs that should be used as rent per block.
    type RentFraction: 'static + scale::Codec + Clone + PartialEq + Eq + Ord + PerThing;
}

/// Placeholder for chains that have no defined chain extension.
pub enum NoChainExtension {}

/// The fundamental types of the default configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(TypeInfo))]
pub enum DefaultEnvironment {}

impl Environment for DefaultEnvironment {
    const MAX_EVENT_TOPICS: usize = 4;

    type AccountId = AccountId;
    type Balance = Balance;
    type Hash = Hash;
    type Timestamp = Timestamp;
    type BlockNumber = BlockNumber;
    type ChainExtension = NoChainExtension;
    type RentFraction = RentFraction;
}

/// The default balance type.
pub type Balance = u128;

/// The default timestamp type.
pub type Timestamp = u64;

/// The default block number type.
pub type BlockNumber = u32;

/// The default rent fraction type.
pub type RentFraction = Perbill;

/// The default environment `AccountId` type.
///
/// # Note
///
/// This is a mirror of the `AccountId` type used in the default configuration
/// of PALLET contracts.
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Ord,
    PartialOrd,
    Hash,
    Encode,
    Decode,
    From,
    Default,
)]
#[cfg_attr(feature = "std", derive(TypeInfo))]
pub struct AccountId([u8; 32]);

impl<'a> TryFrom<&'a [u8]> for AccountId {
    type Error = TryFromSliceError;

    fn try_from(bytes: &'a [u8]) -> Result<Self, TryFromSliceError> {
        let address = <[u8; 32]>::try_from(bytes)?;
        Ok(Self(address))
    }
}

/// The default environment `Hash` type.
///
/// # Note
///
/// This is a mirror of the `Hash` type used in the default configuration
/// of PALLET contracts.
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Ord,
    PartialOrd,
    Hash,
    Encode,
    Decode,
    From,
    Default,
)]
#[cfg_attr(feature = "std", derive(TypeInfo))]
pub struct Hash([u8; 32]);

impl<'a> TryFrom<&'a [u8]> for Hash {
    type Error = TryFromSliceError;

    fn try_from(bytes: &'a [u8]) -> Result<Self, TryFromSliceError> {
        let address = <[u8; 32]>::try_from(bytes)?;
        Ok(Self(address))
    }
}

impl AsRef<[u8]> for Hash {
    fn as_ref(&self) -> &[u8] {
        &self.0[..]
    }
}

impl AsMut<[u8]> for Hash {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0[..]
    }
}

/// The equivalent of `Zero` for hashes.
///
/// A hash that consists only of 0 bits is clear.
pub trait Clear {
    /// Returns `true` if the hash is clear.
    fn is_clear(&self) -> bool;

    /// Returns a clear hash.
    fn clear() -> Self;
}

impl Clear for [u8; 32] {
    fn is_clear(&self) -> bool {
        self.as_ref().iter().all(|&byte| byte == 0x00)
    }

    fn clear() -> Self {
        [0x00; 32]
    }
}

impl Clear for Hash {
    fn is_clear(&self) -> bool {
        <[u8; 32] as Clear>::is_clear(&self.0)
    }

    fn clear() -> Self {
        Self(<[u8; 32] as Clear>::clear())
    }
}

/// Information needed for rent calculations that can be requested by a contract.
#[derive(scale::Decode)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct RentParams<T: Environment> {
    /// The total balance of the contract. Includes the balance transferred from the caller.
    pub total_balance: T::Balance,

    /// The free balance of the contract, i.e. the portion of the contract's balance
    /// that is not reserved. Includes the balance transferred from the caller.
    pub free_balance: T::Balance,

    /// Subsistence threshold is the extension of the minimum balance (aka existential deposit)
    /// by the tombstone deposit, required for leaving a tombstone.
    ///
    /// Rent or any contract initiated balance transfer mechanism cannot make the balance lower
    /// than the subsistence threshold in order to guarantee that a tombstone is created.
    ///
    /// The only way to completely kill a contract without a tombstone is calling `seal_terminate`.
    pub subsistence_threshold: T::Balance,

    /// The balance every contract needs to deposit to stay alive indefinitely.
    ///
    /// This is different from the tombstone deposit because this only needs to be
    /// deposited while the contract is alive. Costs for additional storage are added to
    /// this base cost.
    ///
    /// This is a simple way to ensure that contracts with empty storage eventually get deleted by
    /// making them pay rent. This creates an incentive to remove them early in order to save rent.
    pub deposit_per_contract: T::Balance,

    /// The balance a contract needs to deposit per storage byte to stay alive indefinitely.
    ///
    /// Let's suppose the deposit is 1,000 BU (balance units)/byte and the rent is 1 BU/byte/day,
    /// then a contract with 1,000,000 BU that uses 1,000 bytes of storage would pay no rent.
    /// But if the balance reduced to 500,000 BU and the storage stayed the same at 1,000,
    /// then it would pay 500 BU/day.
    pub deposit_per_storage_byte: T::Balance,

    /// The balance a contract needs to deposit per storage item to stay alive indefinitely.
    ///
    /// It works as [`Self::deposit_per_storage_byte`] but for storage items.
    pub deposit_per_storage_item: T::Balance,

    /// The contract's rent allowance, the rent mechanism cannot consume more than this.
    pub rent_allowance: T::Balance,

    /// The fraction of the deposit costs that should be used as rent per block.
    ///
    /// When a contract does not have enough balance deposited to stay alive indefinitely
    /// it needs to pay per block for the storage it consumes that is not covered by the
    /// deposit. This determines how high this rent payment is per block as a fraction
    /// of the deposit costs.
    pub rent_fraction: T::RentFraction,

    /// The total number of bytes used by this contract.
    ///
    /// It is a sum of each key-value pair stored by this contract.
    pub storage_size: u32,

    /// Sum of instrumented and pristine code length.
    pub code_size: u32,

    /// The number of contracts using this executable.
    pub code_refcount: u32,

    /// Reserved for backwards compatible changes to this data structure.
    pub _reserved: Option<()>,
}

/// Information about the required deposit and resulting rent.
///
/// The easiest way to guarantee that a contract stays alive is to assert that
/// `max_rent == 0` at the **end** of a contract's execution.
///
/// # Note
///
/// The `current_*` fields do **not** consider changes to the code's `refcount`
/// made during the currently running call.
#[derive(scale::Decode)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct RentStatus<T: Environment> {
    /// Required deposit assuming that this contract is the only user of its code.
    pub max_deposit: T::Balance,

    /// Required deposit assuming the code's current `refcount`.
    pub current_deposit: T::Balance,

    /// Required deposit assuming the specified `refcount` (`None` if `0` is supplied).
    pub custom_refcount_deposit: Option<T::Balance>,

    /// Rent that is paid assuming that the contract is the only user of its code.
    pub max_rent: T::Balance,

    /// Rent that is paid given the code's current refcount.
    pub current_rent: T::Balance,

    /// Rent that is paid assuming the specified refcount (`None` if `0` is supplied).
    pub custom_refcount_rent: Option<T::Balance>,

    /// Reserved for backwards compatible changes to this data structure.
    pub _reserved: Option<()>,
}