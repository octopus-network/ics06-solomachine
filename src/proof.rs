use crate::cosmos::crypto::PublicKey;
use crate::error::Error;
use crate::prelude::*;
use crate::proof::types::signature_and_data::SignatureAndData;
use tendermint::crypto::signature::Verifier;
use tendermint::Signature;

pub mod types;

// Verify_signature verifies if the the provided public key generated the signature
// over the given data. Single and Multi signature public keys are supported.
// The signature data type must correspond to the public key type. An error is
// returned if signature verification fails or an invalid SignatureData type is
// provided.
// todo (davirain) ref: https://github.com/cosmos/ibc-go/blob/6f1d8d672705c6e8f5b74a396d883e2834a6b943/modules/light-clients/06-solomachine/types/proof.go#L22
pub fn verify_signature(
    publik_key: PublicKey,
    sign_bytes: Vec<u8>,
    signature_and_data: SignatureAndData,
) -> Result<(), Error> {
    let signature = Signature::try_from(signature_and_data.signature)
        .map_err(|e| Error::Other(format!("{}", e)))?;
    tendermint::crypto::default::signature::Verifier::verify(
        publik_key.into(),
        &sign_bytes,
        &signature,
    )
    .map_err(|e| Error::Other(format!("{}", e)))
}
