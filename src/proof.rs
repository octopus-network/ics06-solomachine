use crate::cosmos::crypto::PublicKey;
use crate::error::Error;
use crate::signature_and_data::SignatureAndData;
use crate::types::sign_bytes::SignBytes;
use crate::types::DataType;
use alloc::string::String;
use alloc::vec::Vec;
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
