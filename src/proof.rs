use crate::cosmos::crypto::PublicKey;
use crate::error::Error;
use crate::header::Header;
use crate::proof::types::header_data::HeaderData;
use crate::proof::types::sign_bytes::SignBytes;

use crate::signature_and_data::SignatureAndData;
use alloc::string::String;
use alloc::vec::Vec;
use ibc_proto::ibc::core::commitment::v1::MerklePath;
use ibc_proto::protobuf::Protobuf;
use prost::Message;

pub mod types;

// Verify_signature verifies if the the provided public key generated the signature
// over the given data. Single and Multi signature public keys are supported.
// The signature data type must correspond to the public key type. An error is
// returned if signature verification fails or an invalid SignatureData type is
// provided.
// todo (davirain) ref: https://github.com/cosmos/ibc-go/blob/6f1d8d672705c6e8f5b74a396d883e2834a6b943/modules/light-clients/06-solomachine/types/proof.go#L22
pub fn verify_signature(
    _publik_key: PublicKey,
    _sign_bytes: Vec<u8>,
    _signature_and_data: SignatureAndData,
) -> Result<(), Error> {
    todo!()
}
