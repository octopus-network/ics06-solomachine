use super::{
    client_type as sm_client_type, ExecutionContext as SmExecutionContext,
    ValidationContext as SmValidationContext,
};
use crate::cosmos::crypto::PublicKey;
use crate::prelude::*;
use crate::v2::consensus_state::ConsensusState as SmConsensusState;
use crate::v2::error::Error;
use crate::v2::header::Header as SmHeader;
use crate::v2::misbehaviour::Misbehaviour as SmMisbehaviour;
use crate::v2::proof::types::sign_bytes::SignBytes;
use crate::v2::proof::types::signature_and_data::SignatureAndData;
use crate::v2::proof::types::timestamped_signature_data::TimestampedSignatureData;
use crate::v2::proof::types::DataType;
use crate::v2::proof::{
    channel_state_sign_bytes, client_state_sign_bytes, connection_state_sign_bytes,
    consensus_state_sign_bytes, next_sequence_recv_sign_bytes, packet_acknowledgement_sign_bytes,
    packet_commitment_sign_bytes, verify_signature,
};
use ibc::core::ics02_client::client_state::{
    ClientStateCommon, ClientStateExecution, ClientStateValidation, UpdateKind,
};
use ibc::core::ics02_client::client_type::ClientType;
use ibc::core::ics02_client::consensus_state::ConsensusState;
use ibc::core::ics02_client::error::ClientError;
use ibc::core::ics02_client::ClientExecutionContext;
use ibc::core::ics23_commitment::commitment::{
    CommitmentPrefix, CommitmentProofBytes, CommitmentRoot,
};
use ibc::core::ics23_commitment::merkle::apply_prefix;
use ibc::core::ics23_commitment::merkle::MerkleProof;
use ibc::core::ics24_host::identifier::ClientId;
use ibc::core::ics24_host::path::ClientStatePath;
use ibc::core::ics24_host::path::Path;
use ibc::core::timestamp::Timestamp;
use ibc::core::{ExecutionContext, ValidationContext};
use ibc::Height;
use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::lightclients::solomachine::v2::ClientState as RawSmClientState;
use ibc_proto::protobuf::Protobuf;
use prost::Message;
pub mod misbehaviour;
pub mod update_client;

pub const SOLOMACHINE_CLIENT_STATE_TYPE_URL: &str = "/ibc.lightclients.solomachine.v2.ClientState";

/// ClientState defines a solo machine client that tracks the current consensus
/// state and if the client is frozen.
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Debug)]
pub struct ClientState {
    /// latest sequence of the client state
    pub sequence: Height,
    /// frozen sequence of the solo machine
    pub is_frozen: bool,
    pub consensus_state: SmConsensusState,
    /// when set to true, will allow governance to update a solo machine client.
    /// The client will be unfrozen if it is frozen.
    pub allow_update_after_proposal: bool,
}

impl ClientStateCommon for ClientState {
    fn verify_consensus_state(&self, consensus_state: Any) -> Result<(), ClientError> {
        // let tm_consensus_state = TmConsensusState::try_from(consensus_state)?;
        // if tm_consensus_state.root().is_empty() {
        //     return Err(ClientError::Other {
        //         description: "empty commitment root".into(),
        //     });
        // };

        Ok(())
    }

    fn client_type(&self) -> ClientType {
        // tm_client_type()
        todo!()
    }

    fn latest_height(&self) -> Height {
        // self.latest_height
        todo!()
    }

    fn validate_proof_height(&self, proof_height: Height) -> Result<(), ClientError> {
        // if self.latest_height() < proof_height {
        //     return Err(ClientError::InvalidProofHeight {
        //         latest_height: self.latest_height(),
        //         proof_height,
        //     });
        // }
        // Ok(())
        Ok(())
    }

    fn confirm_not_frozen(&self) -> Result<(), ClientError> {
        // if let Some(frozen_height) = self.frozen_height {
        //     return Err(ClientError::ClientFrozen {
        //         description: format!("the client is frozen at height {frozen_height}"),
        //     });
        // }
        // Ok(())
        Ok(())
    }

    fn expired(&self, elapsed: Duration) -> bool {
        // elapsed > self.trusting_period
        false
    }

    /// Perform client-specific verifications and check all data in the new
    /// client state to be the same across all valid Tendermint clients for the
    /// new chain.
    ///
    /// You can learn more about how to upgrade IBC-connected SDK chains in
    /// [this](https://ibc.cosmos.network/main/ibc/upgrades/quick-guide.html)
    /// guide
    fn verify_upgrade_client(
        &self,
        upgraded_client_state: Any,
        upgraded_consensus_state: Any,
        proof_upgrade_client: CommitmentProofBytes,
        proof_upgrade_consensus_state: CommitmentProofBytes,
        root: &CommitmentRoot,
    ) -> Result<(), ClientError> {
        // // Make sure that the client type is of Tendermint type `ClientState`
        // let upgraded_tm_client_state = Self::try_from(upgraded_client_state.clone())?;

        // // Make sure that the consensus type is of Tendermint type `ConsensusState`
        // TmConsensusState::try_from(upgraded_consensus_state.clone())?;

        // // Make sure the latest height of the current client is not greater then
        // // the upgrade height This condition checks both the revision number and
        // // the height
        // if self.latest_height() >= upgraded_tm_client_state.latest_height {
        //     return Err(UpgradeClientError::LowUpgradeHeight {
        //         upgraded_height: self.latest_height(),
        //         client_height: upgraded_tm_client_state.latest_height,
        //     })?;
        // }

        // // Check to see if the upgrade path is set
        // let mut upgrade_path = self.upgrade_path.clone();
        // if upgrade_path.pop().is_none() {
        //     return Err(ClientError::ClientSpecific {
        //         description: "cannot upgrade client as no upgrade path has been set".to_string(),
        //     });
        // };

        // let upgrade_path_prefix = CommitmentPrefix::try_from(upgrade_path[0].clone().into_bytes())
        //     .map_err(ClientError::InvalidCommitmentProof)?;

        // let last_height = self.latest_height().revision_height();

        // let mut client_state_value = Vec::new();
        // upgraded_client_state
        //     .encode(&mut client_state_value)
        //     .map_err(ClientError::Encode)?;

        // // Verify the proof of the upgraded client state
        // self.verify_membership(
        //     &upgrade_path_prefix,
        //     &proof_upgrade_client,
        //     root,
        //     Path::UpgradeClient(UpgradeClientPath::UpgradedClientState(last_height)),
        //     client_state_value,
        // )?;

        // let mut cons_state_value = Vec::new();
        // upgraded_consensus_state
        //     .encode(&mut cons_state_value)
        //     .map_err(ClientError::Encode)?;

        // // Verify the proof of the upgraded consensus state
        // self.verify_membership(
        //     &upgrade_path_prefix,
        //     &proof_upgrade_consensus_state,
        //     root,
        //     Path::UpgradeClient(UpgradeClientPath::UpgradedClientConsensusState(last_height)),
        //     cons_state_value,
        // )?;

        Ok(())
    }

    fn verify_membership(
        &self,
        prefix: &CommitmentPrefix,
        proof: &CommitmentProofBytes,
        root: &CommitmentRoot,
        path: Path,
        value: Vec<u8>,
    ) -> Result<(), ClientError> {
        // let merkle_path = apply_prefix(prefix, vec![path.to_string()]);
        // let merkle_proof: MerkleProof = RawMerkleProof::try_from(proof.clone())
        //     .map_err(ClientError::InvalidCommitmentProof)?
        //     .into();

        // merkle_proof
        //     .verify_membership(
        //         &self.proof_specs,
        //         root.clone().into(),
        //         merkle_path,
        //         value,
        //         0,
        //     )
        //     .map_err(ClientError::Ics23Verification)
        Ok(())
    }

    fn verify_non_membership(
        &self,
        prefix: &CommitmentPrefix,
        proof: &CommitmentProofBytes,
        root: &CommitmentRoot,
        path: Path,
    ) -> Result<(), ClientError> {
        // let merkle_path = apply_prefix(prefix, vec![path.to_string()]);
        // let merkle_proof: MerkleProof = RawMerkleProof::try_from(proof.clone())
        //     .map_err(ClientError::InvalidCommitmentProof)?
        //     .into();

        // merkle_proof
        //     .verify_non_membership(&self.proof_specs, root.clone().into(), merkle_path)
        //     .map_err(ClientError::Ics23Verification)
        Ok(())
    }
}

impl<ClientValidationContext> ClientStateValidation<ClientValidationContext> for ClientState
where
    ClientValidationContext: SmValidationContext,
{
    fn verify_client_message(
        &self,
        ctx: &ClientValidationContext,
        client_id: &ClientId,
        client_message: Any,
        update_kind: &UpdateKind,
    ) -> Result<(), ClientError> {
        // match update_kind {
        //     UpdateKind::UpdateClient => {
        //         let header = TmHeader::try_from(client_message)?;
        //         self.verify_header(ctx, client_id, header)
        //     }
        //     UpdateKind::SubmitMisbehaviour => {
        //         let misbehaviour = TmMisbehaviour::try_from(client_message)?;
        //         self.verify_misbehaviour(ctx, client_id, misbehaviour)
        //     }
        // }
        todo!()
    }

    fn check_for_misbehaviour(
        &self,
        ctx: &ClientValidationContext,
        client_id: &ClientId,
        client_message: Any,
        update_kind: &UpdateKind,
    ) -> Result<bool, ClientError> {
        // match update_kind {
        //     UpdateKind::UpdateClient => {
        //         let header = TmHeader::try_from(client_message)?;
        //         self.check_for_misbehaviour_update_client(ctx, client_id, header)
        //     }
        //     UpdateKind::SubmitMisbehaviour => {
        //         let misbehaviour = TmMisbehaviour::try_from(client_message)?;
        //         self.check_for_misbehaviour_misbehavior(&misbehaviour)
        //     }
        // }
        todo!()
    }
}

impl<E> ClientStateExecution<E> for ClientState
where
    E: SmExecutionContext,
    <E as ClientExecutionContext>::AnyClientState: From<ClientState>,
    <E as ClientExecutionContext>::AnyConsensusState: From<SmConsensusState>,
{
    fn initialise(
        &self,
        ctx: &mut E,
        client_id: &ClientId,
        consensus_state: Any,
    ) -> Result<(), ClientError> {
        // let tm_consensus_state = TmConsensusState::try_from(consensus_state)?;

        // ctx.store_client_state(ClientStatePath::new(client_id), self.clone().into())?;
        // ctx.store_consensus_state(
        //     ClientConsensusStatePath::new(client_id, &self.latest_height),
        //     tm_consensus_state.into(),
        // )?;

        // Ok(())
        todo!()
    }

    fn update_state(
        &self,
        ctx: &mut E,
        client_id: &ClientId,
        header: Any,
    ) -> Result<Vec<Height>, ClientError> {
        todo!()
        // let header = TmHeader::try_from(header)?;
        // let header_height = header.height();

        // let maybe_existing_consensus_state = {
        //     let path_at_header_height = ClientConsensusStatePath::new(client_id, &header_height);

        //     ctx.consensus_state(&path_at_header_height).ok()
        // };

        // if maybe_existing_consensus_state.is_some() {
        //     // if we already had the header installed by a previous relayer
        //     // then this is a no-op.
        //     //
        //     // Do nothing.
        // } else {
        //     let new_consensus_state = TmConsensusState::from(header.clone());
        //     let new_client_state = self.clone().with_header(header)?;

        //     ctx.store_consensus_state(
        //         ClientConsensusStatePath::new(client_id, &new_client_state.latest_height),
        //         new_consensus_state.into(),
        //     )?;
        //     ctx.store_client_state(ClientStatePath::new(client_id), new_client_state.into())?;
        // }

        // let updated_heights = vec![header_height];
        // Ok(updated_heights)
    }

    fn update_state_on_misbehaviour(
        &self,
        ctx: &mut E,
        client_id: &ClientId,
        _client_message: Any,
        _update_kind: &UpdateKind,
    ) -> Result<(), ClientError> {
        // let frozen_client_state = self.clone().with_frozen_height(Height::min(0));

        // ctx.store_client_state(ClientStatePath::new(client_id), frozen_client_state.into())?;

        // Ok(())
        Ok(())
    }

    // Commit the new client state and consensus state to the store
    fn update_state_on_upgrade(
        &self,
        ctx: &mut E,
        client_id: &ClientId,
        upgraded_client_state: Any,
        upgraded_consensus_state: Any,
    ) -> Result<Height, ClientError> {
        todo!()
        // let mut upgraded_tm_client_state = Self::try_from(upgraded_client_state)?;
        // let upgraded_tm_cons_state = TmConsensusState::try_from(upgraded_consensus_state)?;

        // upgraded_tm_client_state.zero_custom_fields();

        // // Construct new client state and consensus state relayer chosen client
        // // parameters are ignored. All chain-chosen parameters come from
        // // committed client, all client-chosen parameters come from current
        // // client.
        // let new_client_state = ClientState::new(
        //     upgraded_tm_client_state.chain_id,
        //     self.trust_level,
        //     self.trusting_period,
        //     upgraded_tm_client_state.unbonding_period,
        //     self.max_clock_drift,
        //     upgraded_tm_client_state.latest_height,
        //     upgraded_tm_client_state.proof_specs,
        //     upgraded_tm_client_state.upgrade_path,
        //     self.allow_update,
        // )?;

        // // The new consensus state is merely used as a trusted kernel against
        // // which headers on the new chain can be verified. The root is just a
        // // stand-in sentinel value as it cannot be known in advance, thus no
        // // proof verification will pass. The timestamp and the
        // // NextValidatorsHash of the consensus state is the blocktime and
        // // NextValidatorsHash of the last block committed by the old chain. This
        // // will allow the first block of the new chain to be verified against
        // // the last validators of the old chain so long as it is submitted
        // // within the TrustingPeriod of this client.
        // // NOTE: We do not set processed time for this consensus state since
        // // this consensus state should not be used for packet verification as
        // // the root is empty. The next consensus state submitted using update
        // // will be usable for packet-verification.
        // let sentinel_root = "sentinel_root".as_bytes().to_vec();
        // let new_consensus_state = TmConsensusState::new(
        //     sentinel_root.into(),
        //     upgraded_tm_cons_state.timestamp,
        //     upgraded_tm_cons_state.next_validators_hash,
        // );

        // let latest_height = new_client_state.latest_height;

        // ctx.store_client_state(ClientStatePath::new(client_id), new_client_state.into())?;
        // ctx.store_consensus_state(
        //     ClientConsensusStatePath::new(client_id, &latest_height),
        //     new_consensus_state.into(),
        // )?;

        // Ok(latest_height)
    }
}

impl ClientState {
    /// Create a new ClientState Instance.
    pub fn new(
        sequence: Height,
        is_frozen: bool,
        consensus_state: SmConsensusState,
        allow_update_after_proposal: bool,
    ) -> Self {
        Self {
            sequence,
            is_frozen,
            consensus_state,
            allow_update_after_proposal,
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
        height: &Height,
        prefix: &CommitmentPrefix,
        proof: &CommitmentProofBytes,
    ) -> Result<(PublicKey, SignatureAndData, Timestamp, u64), Error> {
        if height.revision_number() != 0 {
            return Err(Error::Other(format!(
                "revision must be 0 for solomachine, got revision-number: {}",
                height.revision_number()
            )));
        }

        // sequence is encoded in the revision height of height struct
        let sequence = height.revision_height();
        if prefix.as_bytes().is_empty() {
            return Err(Error::Other("prefix cannot be empty".into()));
        }

        // todo missing validate check Prefix
        // ref: https://github.com/cosmos/ibc-go/blob/6f1d8d672705c6e8f5b74a396d883e2834a6b943/modules/light-clients/06-solomachine/types/client_state.go#L438

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

        let latest_sequence = self.sequence.revision_height();
        if latest_sequence != sequence {
            return Err(Error::Other(format!(
                "client state sequence != proof sequence ({} != {})",
                latest_sequence, sequence,
            )));
        }

        if self.consensus_state.timestamp > timestamp {
            return Err(Error::Other(format!(
                "the consensus state timestamp is greater than the signature timestamp ({} >= {})",
                self.consensus_state.timestamp, timestamp
            )));
        }

        let public_key = self.consensus_state.public_key();
        Ok((
            public_key,
            signature_and_data,
            timestamp,
            height.revision_height(),
        ))
    }
}

impl Protobuf<RawSmClientState> for ClientState {}

impl TryFrom<RawSmClientState> for ClientState {
    type Error = Error;

    fn try_from(raw: RawSmClientState) -> Result<Self, Self::Error> {
        let sequence = Height::new(0, raw.sequence).map_err(Error::InvalidHeight)?;

        let consensus_state: SmConsensusState = raw
            .consensus_state
            .ok_or(Error::ConsensusStateIsEmpty)?
            .try_into()?;

        Ok(Self {
            sequence,
            is_frozen: raw.is_frozen,
            consensus_state,
            allow_update_after_proposal: raw.allow_update_after_proposal,
        })
    }
}

impl From<ClientState> for RawSmClientState {
    fn from(value: ClientState) -> Self {
        let sequence = value.sequence.revision_height();

        Self {
            sequence,
            is_frozen: value.is_frozen,
            consensus_state: Some(value.consensus_state.into()),
            allow_update_after_proposal: value.allow_update_after_proposal,
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
