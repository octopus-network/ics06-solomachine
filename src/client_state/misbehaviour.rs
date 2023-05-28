use crate::client_state::SignatureAndData;
use crate::consensus_state::ConsensusState as SmConsensusState;
use crate::header::Header as SmHeader;
use crate::misbehaviour::Misbehaviour as SmMisbehaviour;
use crate::prelude::*;
use crate::proof::types::sign_bytes::SignBytes;
use crate::proof::verify_signature;
use ibc::core::ics02_client::error::ClientError;
use ibc::core::timestamp::Timestamp;
use ibc::core::{ics24_host::identifier::ClientId, ValidationContext};
use ibc_proto::ibc::core::commitment::v1::MerklePath;
use ibc_proto::protobuf::Protobuf;
use prost::Message;

use super::ClientState;

impl ClientState {
    // verify_misbehaviour determines whether or not two conflicting headers at
    // the same height would have convinced the light client.
    pub fn verify_misbehaviour(
        &self,
        _ctx: &dyn ValidationContext,
        _client_id: &ClientId,
        misbehaviour: SmMisbehaviour,
    ) -> Result<(), ClientError> {
        // NOTE: a check that the misbehaviour message data are not equal is done by
        // misbehaviour.ValidateBasic which is called by the 02-client keeper.
        // verify first signature
        self.verify_signature_and_data(misbehaviour.clone(), misbehaviour.signature_one.clone())
            .map_err(|_| ClientError::Other {
                description: "failed to verify signature one".into(),
            })?;

        // verify second signature
        self.verify_signature_and_data(misbehaviour.clone(), misbehaviour.signature_two)
            .map_err(|_| ClientError::Other {
                description: "failed to verify signature one".into(),
            })
    }

    // verifySignatureAndData verifies that the currently registered public key has signed
    // over the provided data and that the data is valid. The data is valid if it can be
    // unmarshaled into the specified data type.
    // ref: https://github.com/cosmos/ibc-go/blob/388283012124fd3cd66c9541000541d9c6767117/modules/light-clients/06-solomachine/misbehaviour_handle.go#L41
    pub fn verify_signature_and_data(
        &self,
        misbehaviour: SmMisbehaviour,
        signature_and_data: SignatureAndData,
    ) -> Result<(), ClientError> {
        // do not check misbehaviour timestamp since we want to allow processing of past misbehaviour
        let _merkle_path =
            MerklePath::decode(&*signature_and_data.path).map_err(|e| ClientError::Other {
                description: format!("{}", e),
            })?;
        let sign_bytes = SignBytes {
            sequence: misbehaviour.sequence.revision_height(),
            timestamp: signature_and_data.timestamp.nanoseconds(),
            diversifier: self.consensus_state.diversifier.clone(),
            path: signature_and_data.path,
            data: signature_and_data.data,
        };
        let data = sign_bytes.encode_vec();

        let signature_and_data = SignatureAndData::decode_vec(&signature_and_data.signature)
            .map_err(|_| ClientError::Other {
                description: "failed to decode SignatureData".into(),
            })?;

        let public_key = self.consensus_state.public_key();

        verify_signature(public_key, data, signature_and_data).map_err(|e| ClientError::Other {
            description: e.to_string(),
        })
    }

    pub fn verify_misbehaviour_header(
        &self,
        _header: &SmHeader,
        _trusted_consensus_state: &SmConsensusState,
        _current_timestamp: Timestamp,
    ) -> Result<(), ClientError> {
        Ok(())
    }

    pub fn check_for_misbehaviour_misbehavior(
        &self,
        _misbehaviour: &SmMisbehaviour,
    ) -> Result<bool, ClientError> {
        Ok(true)
    }
}
