use crate::cosmos::crypto::PublicKey;
use crate::prelude::*;
use crate::v3::error::Error;
use ibc::core::ics02_client::error::ClientError;
use ibc::core::ics23_commitment::commitment::CommitmentRoot;
use ibc::core::timestamp::Timestamp;
use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::lightclients::solomachine::v3::ConsensusState as RawSmConsensusState;
use ibc_proto::protobuf::Protobuf;
use prost::Message;

pub const SOLOMACHINE_CONSENSUS_STATE_TYPE_URL: &str =
    "/ibc.lightclients.solomachine.v3.ConsensusState";

/// ConsensusState defines a solo machine consensus state. The sequence of a
/// consensus state is contained in the "height" key used in storing the
/// consensus state.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, PartialEq, Debug)]
pub struct ConsensusState {
    /// public key of the solo machine
    pub public_key: PublicKey,
    /// diversifier allows the same public key to be re-used across different solo
    /// machine clients (potentially on different chains) without being considered
    /// misbehaviour.
    pub diversifier: String,
    pub timestamp: Timestamp,
    // todo(davirian), this can improve
    root: CommitmentRoot,
}

impl ConsensusState {
    pub fn new(public_key: PublicKey, diversifier: String, timestamp: Timestamp) -> Self {
        Self {
            public_key,
            diversifier,
            timestamp,
            root: CommitmentRoot::from(public_key.to_bytes()),
        }
    }

    // ValidateBasic defines basic validation for the solo machine consensus state.
    pub fn valida_basic(&self) -> Result<(), Error> {
        if self.timestamp.into_tm_time().is_none() {
            return Err(Error::TimeStampIsEmpty);
        }

        if !self.diversifier.is_empty() && self.diversifier.trim().is_empty() {
            return Err(Error::DriversifierContainOnlySpaces);
        }

        let _ = self.public_key();

        Ok(())
    }

    // GetPubKey unmarshals the public key into a cryptotypes.PubKey type.
    // An error is returned if the public key is nil or the cached value
    // is not a PubKey.
    // todo(davirain)
    pub fn public_key(&self) -> PublicKey {
        self.public_key
    }
}

impl ibc::core::ics02_client::consensus_state::ConsensusState for ConsensusState {
    fn root(&self) -> &CommitmentRoot {
        &self.root
    }

    fn timestamp(&self) -> Timestamp {
        self.timestamp
    }

    /// Serializes the `ConsensusState`. This is expected to be implemented as
    /// first converting to the raw type (i.e. the protobuf definition), and then
    /// serializing that.
    ///
    /// Note that the `Protobuf` trait in `tendermint-proto` provides convenience methods
    /// to do this automatically.
    fn encode_vec(&self) -> Vec<u8> {
        <Self as Protobuf<Any>>::encode_vec(self)
    }
}

impl Protobuf<RawSmConsensusState> for ConsensusState {}

impl TryFrom<RawSmConsensusState> for ConsensusState {
    type Error = Error;

    fn try_from(raw: RawSmConsensusState) -> Result<Self, Self::Error> {
        let public_key = PublicKey::try_from(raw.public_key.ok_or(Error::PublicKeyIsEmpty)?)
            .map_err(Error::PublicKeyParseFailed)?;
        let timestamp =
            Timestamp::from_nanoseconds(raw.timestamp).map_err(Error::ParseTimeError)?;
        Ok(Self {
            public_key,
            diversifier: raw.diversifier,
            timestamp,
            root: CommitmentRoot::from(vec![]),
        })
    }
}

impl From<ConsensusState> for RawSmConsensusState {
    fn from(value: ConsensusState) -> Self {
        let public_key = value.public_key.to_any();
        let timestamp = value.timestamp.nanoseconds();
        Self {
            public_key: Some(public_key),
            diversifier: value.diversifier,
            timestamp,
        }
    }
}

impl Protobuf<Any> for ConsensusState {}

impl TryFrom<Any> for ConsensusState {
    type Error = ClientError;

    fn try_from(raw: Any) -> Result<Self, Self::Error> {
        use bytes::Buf;
        use core::ops::Deref;

        fn decode_consensus_state<B: Buf>(buf: B) -> Result<ConsensusState, Error> {
            RawSmConsensusState::decode(buf)
                .map_err(Error::Decode)?
                .try_into()
        }

        match raw.type_url.as_str() {
            SOLOMACHINE_CONSENSUS_STATE_TYPE_URL => {
                decode_consensus_state(raw.value.deref()).map_err(Into::into)
            }
            _ => Err(ClientError::UnknownConsensusStateType {
                consensus_state_type: raw.type_url,
            }),
        }
    }
}

impl From<ConsensusState> for Any {
    fn from(consensus_state: ConsensusState) -> Self {
        Any {
            type_url: SOLOMACHINE_CONSENSUS_STATE_TYPE_URL.to_string(),
            value: Protobuf::<RawSmConsensusState>::encode_vec(&consensus_state),
        }
    }
}
