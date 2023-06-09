use crate::cosmos::crypto::PublicKey;
use crate::prelude::*;
use crate::v3::consensus_state::ConsensusState as SmConsensusState;
use crate::v3::error::Error;
use crate::v3::header::Header as SmHeader;
use crate::v3::misbehaviour::Misbehaviour as SmMisbehaviour;
use crate::v3::proof::types::sign_bytes::SignBytes;
use crate::v3::proof::types::signature_and_data::SignatureAndData;
use crate::v3::proof::types::timestamped_signature_data::TimestampedSignatureData;
use crate::v3::proof::verify_signature;
use ibc::core::ics02_client::client_state::UpdateKind;
use ibc::core::ics02_client::client_state::{ClientState as Ics2ClientState, UpdatedState};
use ibc::core::ics02_client::client_type::ClientType;
use ibc::core::ics02_client::consensus_state::ConsensusState;
use ibc::core::ics02_client::error::ClientError;
use ibc::core::ics23_commitment::commitment::{
    CommitmentPrefix, CommitmentProofBytes, CommitmentRoot,
};
use ibc::core::ics23_commitment::merkle::apply_prefix;
use ibc::core::ics23_commitment::merkle::MerkleProof;
use ibc::core::ics24_host::identifier::{ChainId, ClientId};
use ibc::core::ics24_host::path::ClientStatePath;
use ibc::core::ics24_host::path::Path;
use ibc::core::timestamp::Timestamp;
use ibc::core::{ExecutionContext, ValidationContext};
use ibc::Height;
use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::lightclients::solomachine::v3::ClientState as RawSmClientState;
use ibc_proto::protobuf::Protobuf;
use prost::Message;

pub mod misbehaviour;
pub mod update_client;

pub const SOLOMACHINE_CLIENT_STATE_TYPE_URL: &str = "/ibc.lightclients.solomachine.v3.ClientState";

/// ClientState defines a solo machine client that tracks the current consensus
/// state and if the client is frozen.
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Debug)]
pub struct ClientState {
    /// for chain id
    pub chain_id: ChainId,
    /// latest sequence of the client state
    pub sequence: Height,
    /// frozen sequence of the solo machine
    pub is_frozen: bool,
    pub consensus_state: SmConsensusState,
}

impl ClientState {
    /// Create a new ClientState Instance.
    pub fn new(
        chain_id: ChainId,
        sequence: Height,
        is_frozen: bool,
        consensus_state: SmConsensusState,
    ) -> Self {
        Self {
            chain_id,
            sequence,
            is_frozen,
            consensus_state,
        }
    }

    pub fn with_frozen(self) -> Self {
        Self {
            is_frozen: true,
            ..self
        }
    }

    /// Return exported.Height to satisfy ClientState interface
    /// Revision number is always 0 for a solo-machine.
    pub fn latest_height(&self) -> Height {
        self.sequence
    }

    // GetTimestampAtHeight returns the timestamp in nanoseconds of the consensus state at the given height.
    pub fn time_stamp(&self) -> Timestamp {
        self.consensus_state.timestamp
    }

    // Validate performs basic validation of the client state fields.
    pub fn valida_basic(&self) -> Result<(), Error> {
        if self.sequence.revision_height() == 0 {
            return Err(Error::SequenceCannotZero);
        }
        self.consensus_state.valida_basic()
    }

    // produceVerificationArgs perfoms the basic checks on the arguments that are
    // shared between the verification functions and returns the public key of the
    // consensus state, the unmarshalled proof representing the signature and timestamp.
    pub fn produce_verification_args(
        &self,
        proof: &CommitmentProofBytes,
    ) -> Result<(PublicKey, SignatureAndData, Timestamp, u64), Error> {
        let proof = Vec::<u8>::from(proof.clone());
        if proof.is_empty() {
            return Err(Error::Other("proof cannot be empty".into()));
        }

        let timestamped_sig_data = TimestampedSignatureData::decode_vec(&proof).map_err(|e| {
            Error::Other(format!(
                "failed to decode proof into type TimestampedSignatureData: {}",
                e
            ))
        })?;

        let timestamp = timestamped_sig_data.timestamp;
        let signature_and_data = timestamped_sig_data.signature_data;

        if self.consensus_state.timestamp > timestamp {
            return Err(Error::Other(format!(
                "the consensus state timestamp is greater than the signature timestamp ({} >= {})",
                self.consensus_state.timestamp, timestamp
            )));
        }

        let latest_sequence = self.sequence.revision_height();
        let public_key = self.consensus_state.public_key();
        Ok((public_key, signature_and_data, timestamp, latest_sequence))
    }
}

impl Ics2ClientState for ClientState {
    /// ClientType is Solo Machine.
    fn client_type(&self) -> ClientType {
        crate::v3::client_type()
    }

    /// latest_height returns the latest sequence number.
    fn latest_height(&self) -> Height {
        self.latest_height()
    }

    /// Check if the given proof has a valid height for the client
    fn validate_proof_height(&self, proof_height: Height) -> Result<(), ClientError> {
        if self.latest_height() < proof_height {
            return Err(ClientError::InvalidProofHeight {
                latest_height: self.latest_height(),
                proof_height,
            });
        }
        Ok(())
    }

    /// Assert that the client is not frozen
    fn confirm_not_frozen(&self) -> Result<(), ClientError> {
        if self.is_frozen {
            return Err(ClientError::ClientFrozen {
                description: "the client is frozen".into(),
            });
        }
        Ok(())
    }

    /// Check if the state is expired when `elapsed` time has passed since the latest consensus
    /// state timestamp
    fn expired(&self, _elapsed: Duration) -> bool {
        // todo(davirian)
        false
    }

    fn initialise(&self, consensus_state: Any) -> Result<Box<dyn ConsensusState>, ClientError> {
        SmConsensusState::try_from(consensus_state).map(SmConsensusState::into_box)
    }

    /// verify_client_message must verify a client_message. A client_message
    /// could be a Header, Misbehaviour. It must handle each type of
    /// client_message appropriately. Calls to check_for_misbehaviour,
    /// update_state, and update_state_on_misbehaviour will assume that the
    /// content of the client_message has been verified and can be trusted. An
    /// error should be returned if the client_message fails to verify.
    ///
    /// VerifyClientMessage introspects the provided ClientMessage and checks its validity
    /// A Solomachine Header is considered valid if the currently registered public key has signed over
    /// the new public key with the correct sequence.
    /// A Solomachine Misbehaviour is considered valid if duplicate signatures of the current public key
    /// are found on two different messages at a given sequence.
    fn verify_client_message(
        &self,
        ctx: &dyn ValidationContext,
        client_id: &ClientId,
        client_message: Any,
        update_kind: &UpdateKind,
    ) -> Result<(), ClientError> {
        match update_kind {
            UpdateKind::UpdateClient => {
                let header = SmHeader::try_from(client_message)?;
                self.verify_header(ctx, client_id, header)
            }
            UpdateKind::SubmitMisbehaviour => {
                let misbehaviour = SmMisbehaviour::try_from(client_message)?;
                self.verify_misbehaviour(ctx, client_id, misbehaviour)
            }
        }
    }

    /// Checks for evidence of a misbehaviour in Header or Misbehaviour type. It
    /// assumes the client_message has already been verified.
    fn check_for_misbehaviour(
        &self,
        ctx: &dyn ValidationContext,
        client_id: &ClientId,
        client_message: Any,
        update_kind: &UpdateKind,
    ) -> Result<bool, ClientError> {
        match update_kind {
            UpdateKind::UpdateClient => {
                let header = SmHeader::try_from(client_message)?;
                self.check_for_misbehaviour_update_client(ctx, client_id, header)
            }
            UpdateKind::SubmitMisbehaviour => {
                let misbehaviour = SmMisbehaviour::try_from(client_message)?;
                self.check_for_misbehaviour_misbehavior(&misbehaviour)
            }
        }
    }

    /// Updates and stores as necessary any associated information for an IBC
    /// client, such as the ClientState and corresponding ConsensusState. Upon
    /// successful update, a list of consensus heights is returned. It assumes
    /// the client_message has already been verified.
    ///
    /// Post-condition: on success, the return value MUST contain at least one
    /// height.
    // ref: https://github.com/cosmos/ibc-go/blob/388283012124fd3cd66c9541000541d9c6767117/modules/light-clients/06-solomachine/update.go#L80
    fn update_state(
        &self,
        ctx: &mut dyn ExecutionContext,
        client_id: &ClientId,
        header: Any,
    ) -> Result<Vec<Height>, ClientError> {
        let sm_header = SmHeader::try_from(header).map_err(|e| ClientError::Other {
            description: format!("decode SmHeader Error({})", e),
        })?;

        let consensus_state = SmConsensusState::new(
            sm_header.new_public_key,
            sm_header.new_diversifier,
            sm_header.timestamp,
        );
        let mut new_client_state = self.clone();
        new_client_state.sequence.increment();
        let new_height = new_client_state.sequence;
        new_client_state.consensus_state = consensus_state;

        ctx.store_client_state(ClientStatePath::new(client_id), new_client_state.into_box())?;

        Ok(vec![new_height])
    }

    /// update_state_on_misbehaviour should perform appropriate state changes on
    /// a client state given that misbehaviour has been detected and verified
    fn update_state_on_misbehaviour(
        &self,
        ctx: &mut dyn ExecutionContext,
        client_id: &ClientId,
        _client_message: Any,
        _update_kind: &UpdateKind,
    ) -> Result<(), ClientError> {
        let frozen_client_state = self.clone().with_frozen().into_box();

        ctx.store_client_state(ClientStatePath::new(client_id), frozen_client_state)?;

        Ok(())
    }

    /// Verify the upgraded client and consensus states and validate proofs
    /// against the given root.
    ///
    /// NOTE: proof heights are not included as upgrade to a new revision is
    /// expected to pass only on the last height committed by the current
    /// revision. Clients are responsible for ensuring that the planned last
    /// height of the current revision is somehow encoded in the proof
    /// verification process. This is to ensure that no premature upgrades
    /// occur, since upgrade plans committed to by the counterparty may be
    /// cancelled or modified before the last planned height.
    fn verify_upgrade_client(
        &self,
        _upgraded_client_state: Any,
        _upgraded_consensus_state: Any,
        _proof_upgrade_client: MerkleProof,
        _proof_upgrade_consensus_state: MerkleProof,
        _root: &CommitmentRoot,
    ) -> Result<(), ClientError> {
        Ok(())
    }

    // Update the client state and consensus state in the store with the upgraded ones.
    fn update_state_with_upgrade_client(
        &self,
        _upgraded_client_state: Any,
        _upgraded_consensus_state: Any,
    ) -> Result<UpdatedState, ClientError> {
        // ref: https://github.com/cosmos/ibc-go/blob/f32b1052e1357949e6a67685d355c7bcc6242b84/modules/light-clients/06-solomachine/client_state.go#L99
        Err(ClientError::Other {
            description: "cannot upgrade solomachine client".into(),
        })
    }

    // Verify_membership is a generic proof verification method which verifies a
    // proof of the existence of a value at a given Path.
    fn verify_membership(
        &self,
        prefix: &CommitmentPrefix,
        proof: &CommitmentProofBytes,
        _root: &CommitmentRoot,
        path: Path,
        value: Vec<u8>,
    ) -> Result<(), ClientError> {
        let (public_key, sig_data, timestamp, sequence) = self.produce_verification_args(proof)?;
        let merkle_path = apply_prefix(prefix, vec![path.to_string()]);
        if merkle_path.key_path.is_empty() {
            return Err(ClientError::Other {
                description: "path is empty".to_string(),
            });
        }
        let sign_bytes = SignBytes {
            sequence,
            timestamp: timestamp.nanoseconds(),
            diversifier: self.consensus_state.diversifier.clone(),
            path: merkle_path,
            data: value,
        };
        let sign_bz = sign_bytes.encode_vec();

        verify_signature(public_key, sign_bz, sig_data).map_err(|e| ClientError::Other {
            description: e.to_string(),
        })
    }

    // Verify_non_membership is a generic proof verification method which
    // verifies the absence of a given commitment.
    //
    // VerifyNonMembership is a generic proof verification method which verifies the absence
    // of a given CommitmentPath at the latest sequence.
    // The caller is expected to construct the full CommitmentPath from a CommitmentPrefix
    // and a standardized path (as defined in ICS 24).
    fn verify_non_membership(
        &self,
        prefix: &CommitmentPrefix,
        proof: &CommitmentProofBytes,
        _root: &CommitmentRoot,
        path: Path,
    ) -> Result<(), ClientError> {
        let (public_key, sig_data, timestamp, sequence) = self.produce_verification_args(proof)?;
        let merkle_path = apply_prefix(prefix, vec![path.to_string()]);
        if merkle_path.key_path.is_empty() {
            return Err(ClientError::Other {
                description: "path is empty".to_string(),
            });
        }
        let sign_bytes = SignBytes {
            sequence,
            timestamp: timestamp.nanoseconds(),
            diversifier: self.consensus_state.diversifier.clone(),
            path: merkle_path,
            data: vec![],
        };
        let sign_bz = sign_bytes.encode_vec();

        verify_signature(public_key, sign_bz, sig_data).map_err(|e| ClientError::Other {
            description: e.to_string(),
        })
    }
}

impl Protobuf<RawSmClientState> for ClientState {}

impl TryFrom<RawSmClientState> for ClientState {
    type Error = Error;

    fn try_from(raw: RawSmClientState) -> Result<Self, Self::Error> {
        let chain_id = ChainId::from_string(raw.chain_id.as_str());
        let sequence = Height::new(0, raw.sequence).map_err(Error::InvalidHeight)?;
        let consensus_state: SmConsensusState = raw
            .consensus_state
            .ok_or(Error::ConsensusStateIsEmpty)?
            .try_into()?;

        Ok(Self {
            chain_id,
            sequence,
            is_frozen: raw.is_frozen,
            consensus_state,
        })
    }
}

impl From<ClientState> for RawSmClientState {
    fn from(value: ClientState) -> Self {
        Self {
            chain_id: value.chain_id.to_string(),
            sequence: value.sequence.revision_height(),
            is_frozen: value.is_frozen,
            consensus_state: Some(value.consensus_state.into()),
        }
    }
}

impl Protobuf<Any> for ClientState {}

impl TryFrom<Any> for ClientState {
    type Error = ClientError;

    fn try_from(raw: Any) -> Result<Self, Self::Error> {
        use bytes::Buf;
        use core::ops::Deref;

        fn decode_client_state<B: Buf>(buf: B) -> Result<ClientState, Error> {
            RawSmClientState::decode(buf)
                .map_err(Error::Decode)?
                .try_into()
        }

        match raw.type_url.as_str() {
            SOLOMACHINE_CLIENT_STATE_TYPE_URL => {
                decode_client_state(raw.value.deref()).map_err(Into::into)
            }
            _ => Err(ClientError::UnknownClientStateType {
                client_state_type: raw.type_url,
            }),
        }
    }
}

impl From<ClientState> for Any {
    fn from(client_state: ClientState) -> Self {
        Any {
            type_url: SOLOMACHINE_CLIENT_STATE_TYPE_URL.to_string(),
            value: Protobuf::<RawSmClientState>::encode_vec(&client_state),
        }
    }
}
