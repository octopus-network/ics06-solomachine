use crate::client_state::ClientState;
use crate::consensus_state::ConsensusState;
use crate::cosmos::crypto::PublicKey;
use crate::error::Error;
use crate::header::Header;
use crate::signature_and_data::SignatureAndData;
use crate::types::client_state_data::ClientStateData;
use crate::types::consensus_state_data::ConsensusStateData;
use crate::types::header_data::HeaderData;
use crate::types::sign_bytes::{self, SignBytes};
use crate::types::DataType;
use alloc::string::String;
use alloc::vec::Vec;
use ibc::core::ics24_host::path::Path;
use ibc_proto::protobuf::Protobuf;

// Verify_signature verifies if the the provided public key generated the signature
// over the given data. Single and Multi signature public keys are supported.
// The signature data type must correspond to the public key type. An error is
// returned if signature verification fails or an invalid SignatureData type is
// provided.
pub fn verify_signature(
    _publik_key: PublicKey,
    _sign_bytes: Vec<u8>,
    _signature_and_data: SignatureAndData,
) -> Result<(), Error> {
    todo!()
}

// MisbehaviourSignBytes returns the sign bytes for verification of misbehaviour.
pub fn misbehaviour_sign_bytes(
    sequence: u64,
    timestamp: u64,
    diversifier: String,
    data_type: DataType,
    data: Vec<u8>,
) -> Vec<u8> {
    let sign_bytest = SignBytes {
        sequence,
        timestamp,
        diversifier,
        data_type,
        data,
    };

    sign_bytest.encode_vec()
}

// HeaderSignBytes returns the sign bytes for verification of misbehaviour.
pub fn header_sign_bytes(heder: Header) -> Vec<u8> {
    let data = HeaderData {
        new_pub_key: heder.new_public_key,
        new_diversifier: heder.new_diversifier.clone(),
    };
    let data_bz = data.encode_vec();

    let sign_bytes = SignBytes {
        sequence: heder.sequence.revision_height(),
        timestamp: heder.timestamp.nanoseconds(),
        diversifier: heder.new_diversifier,
        data_type: DataType::Header,
        data: data_bz,
    };
    sign_bytes.encode_vec()
}

// ClientStateSignBytes returns the sign bytes for verification of the
// client state.
pub fn client_state_sign_bytes(
    sequence: u64,
    timestamp: u64,
    diversifier: String,
    path: Path,
    client_state: ClientState,
) -> Vec<u8> {
    let data_bz = client_state_data_bytes(path, client_state);
    let sign_bytes = SignBytes {
        sequence,
        timestamp,
        diversifier,
        data_type: DataType::ClientState,
        data: data_bz,
    };
    sign_bytes.encode_vec()
}

// ClientStateDataBytes returns the client state data bytes used in constructing
// SignBytes.
pub fn client_state_data_bytes(path: Path, client_state: ClientState) -> Vec<u8> {
    let data = ClientStateData {
        path: alloc::format!("{}", path).as_bytes().to_vec(),
        client_state,
    };
    data.encode_vec()
}

// ConsensusStateSignBytes returns the sign bytes for verification of the
// consensus state.
pub fn consensus_state_sign_bytes(
    sequence: u64,
    timestamp: u64,
    diversifier: String,
    path: Path,
    consensus_state: ConsensusState,
) -> Vec<u8> {
    let data_bz = consensus_state_data_bytes(path, consensus_state);
    let sign_bytes = SignBytes {
        sequence,
        timestamp,
        diversifier,
        data_type: DataType::ConsensusState,
        data: data_bz,
    };
    sign_bytes.encode_vec()
}

// ConsensusStateDataBytes returns the consensus state data bytes used in constructing
// SignBytes.
pub fn consensus_state_data_bytes(path: Path, consensus_state: ConsensusState) -> Vec<u8> {
    let data = ConsensusStateData {
        path: alloc::format!("{}", path).as_bytes().to_vec(),
        consensus_state,
    };
    data.encode_vec()
}
