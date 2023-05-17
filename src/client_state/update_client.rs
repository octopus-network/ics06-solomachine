use ibc_proto::protobuf::Protobuf;

use crate::prelude::*;
use crate::types::header_data::HeaderData;
use crate::types::sign_bytes::SignBytes;

use super::ClientState;
use crate::header::Header as SmHeader;
use crate::proof::verify_signature;
use crate::signature_and_data::SignatureAndData;
use ibc::core::ics02_client::error::ClientError;
use ibc::core::{ics24_host::identifier::ClientId, ValidationContext};
impl ClientState {
    pub fn verify_header(
        &self,
        _ctx: &dyn ValidationContext,
        _client_id: &ClientId,
        header: SmHeader,
    ) -> Result<(), ClientError> {
        // assert update timestamp is not less than current consensus state timestamp
        if header.timestamp < self.consensus_state.timestamp {
            return Err(ClientError::Other {
                description: format!(
                    "header timestamp is less than to the consensus state timestamp ({} < {})",
                    header.timestamp, self.consensus_state.timestamp,
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
            sequence: self.sequence.revision_height(),
            timestamp: header.timestamp,
            diversifier: self.consensus_state.diversifier.clone(),
            // todo(davirain): https://github.com/cosmos/ibc-go/blob/388283012124fd3cd66c9541000541d9c6767117/modules/light-clients/06-solomachine/update.go#LL52C36-L52C36
            data_type: crate::types::DataType::Header,
            data: data_bz,
        };
        let data = sign_bytes.encode_vec();
        let sig_data =
            SignatureAndData::decode_vec(&header.signature).map_err(|_| ClientError::Other {
                description: "failed to decode SignatureData".into(),
            })?;

        let public_key = self.consensus_state.public_key();

        verify_signature(public_key, data, sig_data).map_err(|e| ClientError::Other {
            description: e.to_string(),
        })
    }

    pub fn check_for_misbehaviour_update_client(
        &self,
        _ctx: &dyn ValidationContext,
        _client_id: &ClientId,
        _header: SmHeader,
    ) -> Result<bool, ClientError> {
        todo!()
    }
}
