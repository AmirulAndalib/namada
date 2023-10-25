//! Types that are used in validity predicates.

use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

use crate::proto::Tx;
use crate::types::hash::Hash;

/// A validity predicate with an input that is intended to be invoked via `eval`
/// host function.
#[derive(
    Debug, Clone, BorshSerialize, BorshDeserialize, Serialize, Deserialize,
)]
pub struct EvalVp {
    /// The VP code hash to `eval`
    pub vp_code_hash: Hash,
    /// The input for the `eval`ed VP
    pub input: Tx,
}

/// Sentinel used in validity predicates to signal events that require special
/// replay protection handling back to the protocol.
#[derive(Debug, Default)]
pub enum VpSentinel {
    /// No action required
    #[default]
    None,
    /// Exceeded gas limit
    OutOfGas,
    /// Found invalid transaction signature
    InvalidSignature,
}

impl VpSentinel {
    /// Check if the Vp ran out of gas
    pub fn is_out_of_gas(&self) -> bool {
        matches!(self, Self::OutOfGas)
    }

    /// Check if the Vp found an invalid signature
    pub fn is_invalid_signature(&self) -> bool {
        matches!(self, Self::InvalidSignature)
    }

    /// Set the sentinel for an out of gas error
    pub fn set_out_of_gas(&mut self) {
        *self = Self::OutOfGas
    }

    /// Set the sentinel for an invalid signature error
    pub fn set_invalid_signature(&mut self) {
        *self = Self::InvalidSignature
    }
}
