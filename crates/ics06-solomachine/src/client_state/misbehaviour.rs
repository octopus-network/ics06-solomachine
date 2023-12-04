use crate::proof::verify_signature;
use ibc_client_solomachine_types::proof::types::sign_bytes::SignBytes;
use ibc_client_solomachine_types::proof::types::signature_and_data::SignatureAndData;
use ibc_client_solomachine_types::{Header as SmHeader, Misbehaviour as SmMisbehaviour};
use ibc_core::client::types::error::ClientError;
use ibc_core::host::types::identifiers::ClientId;
use ibc_core::primitives::prelude::*;
use ibc_core::primitives::Timestamp;
use ibc_proto::Protobuf;

use super::{ClientState as SmClientState, SmValidationContext};
use crate::consensus_state::ConsensusState as SmConsensusState;

impl SmClientState {
    // verify_misbehaviour determines whether or not two conflicting headers at
    // the same height would have convinced the light client.
    pub fn verify_misbehaviour<ClientValidationContext>(
        &self,
        _ctx: &ClientValidationContext,
        _client_id: &ClientId,
        misbehaviour: SmMisbehaviour,
    ) -> Result<(), ClientError>
    where
        ClientValidationContext: SmValidationContext,
    {
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
        let sign_bytes = SignBytes {
            sequence: misbehaviour.sequence.revision_height(),
            timestamp: signature_and_data.timestamp.nanoseconds(),
            diversifier: self.0.consensus_state.diversifier.clone(),
            path: signature_and_data.path,
            data: signature_and_data.data,
        };
        let data = sign_bytes.encode_vec();

        let signature_and_data = SignatureAndData::decode_vec(&signature_and_data.signature)
            .map_err(|_| ClientError::Other {
                description: "failed to decode SignatureData".into(),
            })?;

        let public_key = self.0.consensus_state.public_key();

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
        Ok(false)
    }
}
