use crate::proof::verify_signature;
use ibc_client_solomachine_types::proof::types::sign_bytes::SignBytes;
use ibc_client_solomachine_types::Header as SmHeader;
use ibc_core::client::types::error::ClientError;
use ibc_core::host::types::identifiers::ClientId;
use ibc_core::primitives::prelude::*;
use ibc_proto::Protobuf;

use super::ClientState;
use crate::context::ValidationContext as SmValidationContext;
use ibc_client_solomachine_types::proof::types::header_data::HeaderData;
use ibc_client_solomachine_types::proof::types::signature_and_data::SignatureAndData;
use ibc_proto::ibc::core::commitment::v1::MerklePath;

impl ClientState {
    pub fn verify_header<ClientValidationContext>(
        &self,
        _ctx: &ClientValidationContext,
        _client_id: &ClientId,
        header: SmHeader,
    ) -> Result<(), ClientError>
    where
        ClientValidationContext: SmValidationContext,
    {
        // assert update timestamp is not less than current consensus state timestamp
        if header.timestamp < self.0.consensus_state.timestamp {
            return Err(ClientError::Other {
                description: format!(
                    "header timestamp is less than to the consensus state timestamp ({} < {})",
                    header.timestamp, self.0.consensus_state.timestamp,
                ),
            });
        }

        // assert currently registered public key signed over the new public key with correct sequence
        let header_data = HeaderData {
            new_pub_key: header.new_public_key,
            new_diversifier: header.new_diversifier,
        };
        let data_bz = header_data.encode_vec();

        let sign_bytes = SignBytes {
            sequence: self.0.sequence.revision_height(),
            timestamp: header.timestamp.nanoseconds(),
            diversifier: self.0.consensus_state.diversifier.clone(),
            // todo(davirain)
            // ref: https://github.com/cosmos/ibc-go/blob/3765dfc3b89b16c81abcc3e0b1ad5823d7f7eaa0/modules/light-clients/06-solomachine/header.go#L13
            // SentinelHeaderPath defines a placeholder path value used for headers in solomachine client updates
            // const SentinelHeaderPath = "solomachine:header"
            // ref: https://github.com/cosmos/ibc-go/blob/3765dfc3b89b16c81abcc3e0b1ad5823d7f7eaa0/modules/light-clients/06-solomachine/update.go#L48
            path: MerklePath {
                key_path: vec!["solomachine:header".to_string()],
            },
            data: data_bz,
        };
        let data = sign_bytes.encode_vec();
        let sig_data =
            SignatureAndData::decode_vec(&header.signature).map_err(|_| ClientError::Other {
                description: "failed to decode SignatureData".into(),
            })?;

        let public_key = self.0.consensus_state.public_key();

        verify_signature(public_key, data, sig_data).map_err(|e| ClientError::Other {
            description: e.to_string(),
        })
    }

    pub fn check_for_misbehaviour_update_client<ClientValidationContext>(
        &self,
        _ctx: &ClientValidationContext,
        _client_id: &ClientId,
        _header: SmHeader,
    ) -> Result<bool, ClientError>
    where
        ClientValidationContext: SmValidationContext,
    {
        Ok(false)
    }
}
