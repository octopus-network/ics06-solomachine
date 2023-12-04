use alloc::vec::Vec;
use ibc_client_solomachine_types::error::Error;
use ibc_client_solomachine_types::ConsensusState as ConsensusStateType;
use ibc_core::client::types::error::ClientError;
use ibc_core::commitment_types::commitment::CommitmentRoot;
use ibc_core::primitives::Timestamp;
use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::lightclients::solomachine::v3::ConsensusState as RawSmConsensusState;
use ibc_proto::Protobuf;

pub const SOLOMACHINE_CONSENSUS_STATE_TYPE_URL: &str =
    "/ibc.lightclients.solomachine.v3.ConsensusState";

/// Newtype wrapper around the `ConsensusState` type imported from the
/// `ibc-client-tendermint-types` crate. This wrapper exists so that we can
/// bypass Rust's orphan rules and implement traits from
/// `ibc::core::client::context` on the `ConsensusState` type.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct ConsensusState(ConsensusStateType);

impl ConsensusState {
    pub fn inner(&self) -> &ConsensusStateType {
        &self.0
    }
}

impl From<ConsensusStateType> for ConsensusState {
    fn from(consensus_state: ConsensusStateType) -> Self {
        Self(consensus_state)
    }
}

impl Protobuf<RawSmConsensusState> for ConsensusState {}

impl TryFrom<RawSmConsensusState> for ConsensusState {
    type Error = Error;

    fn try_from(raw: RawSmConsensusState) -> Result<Self, Self::Error> {
        Ok(Self(ConsensusStateType::try_from(raw)?))
    }
}

impl From<ConsensusState> for RawSmConsensusState {
    fn from(consensus_state: ConsensusState) -> Self {
        consensus_state.0.into()
    }
}

impl Protobuf<Any> for ConsensusState {}

impl TryFrom<Any> for ConsensusState {
    type Error = ClientError;

    fn try_from(raw: Any) -> Result<Self, Self::Error> {
        Ok(Self(ConsensusStateType::try_from(raw)?))
    }
}

impl From<ConsensusState> for Any {
    fn from(client_state: ConsensusState) -> Self {
        client_state.0.into()
    }
}

impl ibc_core::client::context::consensus_state::ConsensusState for ConsensusState {
    fn root(&self) -> &CommitmentRoot {
        &self.0.root
    }

    fn timestamp(&self) -> Timestamp {
        self.0.timestamp
    }

    /// Serializes the `ConsensusState`. This is expected to be implemented as
    /// first converting to the raw type (i.e. the protobuf definition), and then
    /// serializing that.
    ///
    /// Note that the `Protobuf` trait in `tendermint-proto` provides convenience methods
    /// to do this automatically.
    fn encode_vec(self) -> Vec<u8> {
        <Self as Protobuf<Any>>::encode_vec(self)
    }
}
