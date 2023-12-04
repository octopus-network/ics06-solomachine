use crate::alloc::string::ToString;
use crate::proof::verify_signature;
use alloc::format;
use alloc::vec;
use alloc::vec::Vec;
use ibc_client_solomachine_types::error::Error;
use ibc_client_solomachine_types::proof::types::sign_bytes::SignBytes;
use ibc_client_solomachine_types::{
    client_type as sm_client_type, ClientState as ClientStateType,
    ConsensusState as ConsensusStateType, Header as SmHeader, Misbehaviour as SmMisbehaviour,
};
use ibc_core::client::context::client_state::{
    ClientStateCommon, ClientStateExecution, ClientStateValidation,
};
use ibc_core::client::context::consensus_state::ConsensusState;
use ibc_core::client::context::{ClientExecutionContext, ClientValidationContext};
use ibc_core::client::types::error::ClientError;
use ibc_core::client::types::{Height, Status, UpdateKind};
use ibc_core::commitment_types::commitment::{
    CommitmentPrefix, CommitmentProofBytes, CommitmentRoot,
};
use ibc_core::commitment_types::merkle::apply_prefix;
use ibc_core::host::types::identifiers::{ClientId, ClientType};
use ibc_core::host::types::path::Path;
use ibc_core::host::types::path::{ClientConsensusStatePath, ClientStatePath};
use ibc_core::host::ExecutionContext;
use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::lightclients::solomachine::v3::ClientState as RawSmClientState;
use ibc_proto::Protobuf;

pub mod misbehaviour;
pub mod update_client;

use super::consensus_state::ConsensusState as SmConsensusState;
use crate::context::{
    ExecutionContext as SmExecutionContext, ValidationContext as SmValidationContext,
};

pub const SOLOMACHINE_CLIENT_STATE_TYPE_URL: &str = "/ibc.lightclients.solomachine.v3.ClientState";

/// ClientState defines a solo machine client that tracks the current consensus
/// state and if the client is frozen.
/// Newtype wrapper around the `ClientState` type imported from the
/// `ibc-client-tendermint-types` crate. This wrapper exists so that we can
/// bypass Rust's orphan rules and implement traits from
/// `ibc::core::client::context` on the `ClientState` type.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct ClientState(ClientStateType);

impl ClientState {
    pub fn inner(&self) -> &ClientStateType {
        &self.0
    }
}

impl From<ClientStateType> for ClientState {
    fn from(client_state: ClientStateType) -> Self {
        Self(client_state)
    }
}

impl Protobuf<RawSmClientState> for ClientState {}

impl TryFrom<RawSmClientState> for ClientState {
    type Error = Error;

    fn try_from(raw: RawSmClientState) -> Result<Self, Self::Error> {
        Ok(Self(ClientStateType::try_from(raw)?))
    }
}

impl From<ClientState> for RawSmClientState {
    fn from(client_state: ClientState) -> Self {
        client_state.0.into()
    }
}

impl Protobuf<Any> for ClientState {}

impl TryFrom<Any> for ClientState {
    type Error = ClientError;

    fn try_from(raw: Any) -> Result<Self, Self::Error> {
        Ok(Self(ClientStateType::try_from(raw)?))
    }
}

impl From<ClientState> for Any {
    fn from(client_state: ClientState) -> Self {
        client_state.0.into()
    }
}

impl ClientStateCommon for ClientState {
    fn verify_consensus_state(&self, consensus_state: Any) -> Result<(), ClientError> {
        let tm_consensus_state = SmConsensusState::try_from(consensus_state)?;
        if tm_consensus_state.root().is_empty() {
            return Err(ClientError::Other {
                description: "empty commitment root".into(),
            });
        };

        Ok(())
    }

    fn client_type(&self) -> ClientType {
        sm_client_type()
    }

    fn latest_height(&self) -> Height {
        self.0.latest_height()
    }

    fn validate_proof_height(&self, proof_height: Height) -> Result<(), ClientError> {
        if self.latest_height() < proof_height {
            return Err(ClientError::InvalidProofHeight {
                latest_height: self.latest_height(),
                proof_height,
            });
        }
        Ok(())
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
        _upgraded_client_state: Any,
        _upgraded_consensus_state: Any,
        _proof_upgrade_client: CommitmentProofBytes,
        _proof_upgrade_consensus_state: CommitmentProofBytes,
        _root: &CommitmentRoot,
    ) -> Result<(), ClientError> {
        // todo: no implement
        Ok(())
    }

    fn verify_membership(
        &self,
        prefix: &CommitmentPrefix,
        proof: &CommitmentProofBytes,
        _root: &CommitmentRoot,
        path: Path,
        value: Vec<u8>,
    ) -> Result<(), ClientError> {
        let (public_key, sig_data, timestamp, sequence) =
            self.0.produce_verification_args(proof)?;
        let merkle_path = apply_prefix(prefix, vec![path.to_string()]);
        if merkle_path.key_path.is_empty() {
            return Err(ClientError::Other {
                description: "path is empty".to_string(),
            });
        }
        let sign_bytes = SignBytes {
            sequence,
            timestamp: timestamp.nanoseconds(),
            diversifier: self.0.consensus_state.diversifier.clone(),
            path: merkle_path,
            data: value,
        };
        let sign_bz = sign_bytes.encode_vec();
        verify_signature(public_key, sign_bz, sig_data).map_err(|e| ClientError::Other {
            description: e.to_string(),
        })
    }

    fn verify_non_membership(
        &self,
        prefix: &CommitmentPrefix,
        proof: &CommitmentProofBytes,
        _root: &CommitmentRoot,
        path: Path,
    ) -> Result<(), ClientError> {
        let (public_key, sig_data, timestamp, sequence) =
            self.0.produce_verification_args(proof)?;
        let merkle_path = apply_prefix(prefix, vec![path.to_string()]);
        if merkle_path.key_path.is_empty() {
            return Err(ClientError::Other {
                description: "path is empty".to_string(),
            });
        }
        let sign_bytes = SignBytes {
            sequence,
            timestamp: timestamp.nanoseconds(),
            diversifier: self.0.consensus_state.diversifier.clone(),
            path: merkle_path,
            data: vec![],
        };
        let sign_bz = sign_bytes.encode_vec();

        verify_signature(public_key, sign_bz, sig_data).map_err(|e| ClientError::Other {
            description: e.to_string(),
        })
    }
}

impl<V> ClientStateValidation<V> for ClientState
where
    V: ClientValidationContext + SmValidationContext,
    V::AnyConsensusState: TryInto<SmConsensusState>,
    ClientError: From<<V::AnyConsensusState as TryInto<SmConsensusState>>::Error>,
{
    fn verify_client_message(
        &self,
        ctx: &V,
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

    fn check_for_misbehaviour(
        &self,
        ctx: &V,
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

    fn status(&self, _ctx: &V, _client_id: &ClientId) -> Result<Status, ClientError> {
        if self.0.is_frozen {
            return Ok(Status::Frozen);
        }

        // let latest_consensus_state: SmConsensusState = {
        //     let any_latest_consensus_state = match ctx
        //         .consensus_state(&ClientConsensusStatePath::new(client_id, &self.sequence))
        //     {
        //         Ok(cs) => cs,
        //         // if the client state does not have an associated consensus state for its latest height
        //         // then it must be expired
        //         Err(_) => return Ok(Status::Expired),
        //     };

        //     any_latest_consensus_state.try_into()?
        // };

        // Note: if the `duration_since()` is `None`, indicating that the latest
        // consensus state is in the future, then we don't consider the client
        // to be expired.
        // let now = ctx.host_timestamp()?;
        // if let Some(elapsed_since_latest_consensus_state) =
        //     now.duration_since(&latest_consensus_state.timestamp())
        // {
        //     if elapsed_since_latest_consensus_state > self.consensus_state.timestamp.into() {
        //         return Ok(Status::Expired);
        //     }
        // }

        Ok(Status::Active)
    }
}

impl<E> ClientStateExecution<E> for ClientState
where
    E: SmExecutionContext + ExecutionContext,
    <E as ClientExecutionContext>::AnyClientState: From<ClientState>,
    <E as ClientExecutionContext>::AnyConsensusState: From<SmConsensusState>,
{
    fn initialise(
        &self,
        ctx: &mut E,
        client_id: &ClientId,
        consensus_state: Any,
    ) -> Result<(), ClientError> {
        let sm_consensus_state = SmConsensusState::try_from(consensus_state)?;

        ctx.store_client_state(ClientStatePath::new(client_id), self.clone().into())?;
        ctx.store_consensus_state(
            ClientConsensusStatePath::new(
                client_id.clone(),
                self.latest_height().revision_number(),
                self.latest_height().revision_height(),
            ),
            sm_consensus_state.into(),
        )?;

        Ok(())
    }

    fn update_state(
        &self,
        ctx: &mut E,
        client_id: &ClientId,
        header: Any,
    ) -> Result<Vec<Height>, ClientError> {
        let sm_header = SmHeader::try_from(header).map_err(|e| ClientError::Other {
            description: format!("decode SmHeader Error({})", e),
        })?;
        let consensus_state = SmConsensusState::from(ConsensusStateType::new(
            sm_header.new_public_key,
            sm_header.new_diversifier,
            sm_header.timestamp,
        ));
        let mut new_client_state = self.clone();
        new_client_state.0.sequence.increment();
        let new_height = new_client_state.0.sequence;
        new_client_state.0.consensus_state = consensus_state.inner().clone();
        ctx.store_client_state(ClientStatePath::new(client_id), new_client_state.into())?;
        Ok(vec![new_height])
    }

    fn update_state_on_misbehaviour(
        &self,
        ctx: &mut E,
        client_id: &ClientId,
        _client_message: Any,
        _update_kind: &UpdateKind,
    ) -> Result<(), ClientError> {
        let frozen_client_state = self.clone().0.with_frozen_height(Height::min(0));

        let wrapped_frozen_client_state = ClientState::from(frozen_client_state);

        ctx.store_client_state(
            ClientStatePath::new(client_id),
            wrapped_frozen_client_state.into(),
        )?;

        Ok(())
    }

    // Commit the new client state and consensus state to the store
    fn update_state_on_upgrade(
        &self,
        _ctx: &mut E,
        _client_id: &ClientId,
        _upgraded_client_state: Any,
        _upgraded_consensus_state: Any,
    ) -> Result<Height, ClientError> {
        Err(ClientError::Other {
            description: "No implement".to_string(),
        })
    }
}
