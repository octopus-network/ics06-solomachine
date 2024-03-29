use super::consensus_state::ConsensusState as SmConsensusState;
use alloc::string::ToString;

use ibc_core::client::context::ClientExecutionContext;
use ibc_core::client::types::Height;
use ibc_core::handler::types::error::ContextError;
use ibc_core::host::types::identifiers::ClientId;
use ibc_core::host::types::path::ClientConsensusStatePath;
use ibc_core::primitives::prelude::*;
use ibc_core::primitives::Timestamp;

/// Client's context required during both validation and execution
pub trait CommonContext {
    type ConversionError: ToString;
    type AnyConsensusState: TryInto<SmConsensusState, Error = Self::ConversionError>;

    /// Retrieve the consensus state for the given client ID at the specified
    /// height.
    ///
    /// Returns an error if no such state exists.
    fn consensus_state(
        &self,
        client_cons_state_path: &ClientConsensusStatePath,
    ) -> Result<Self::AnyConsensusState, ContextError>;
}

/// Client's context required during validation
pub trait ValidationContext: CommonContext {
    /// Returns the current timestamp of the local chain.
    fn host_timestamp(&self) -> Result<Timestamp, ContextError>;

    /// Search for the lowest consensus state higher than `height`.
    fn next_consensus_state(
        &self,
        client_id: &ClientId,
        height: &Height,
    ) -> Result<Option<Self::AnyConsensusState>, ContextError>;

    /// Search for the highest consensus state lower than `height`.
    fn prev_consensus_state(
        &self,
        client_id: &ClientId,
        height: &Height,
    ) -> Result<Option<Self::AnyConsensusState>, ContextError>;
}

/// Client's context required during execution.
///
/// This trait is automatically implemented for all types that implement
/// [`CommonContext`] and [`ClientExecutionContext`]
pub trait ExecutionContext: CommonContext + ClientExecutionContext {}

impl<T> ExecutionContext for T where T: CommonContext + ClientExecutionContext {}
