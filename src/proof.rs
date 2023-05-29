use crate::cosmos::crypto::public_key::{ED25519_TYPE_URL, SECP256K1_TYPE_URL};
use crate::cosmos::crypto::PublicKey;
use crate::error::Error;
use crate::signature_and_data::SignatureAndData;
use alloc::format;
use alloc::vec::Vec;
use ed25519_dalek::Verifier;
use ed25519_dalek::{PublicKey as Ed25519PublicKey, Signature};
use secp256k1::{ecdsa, Message, PublicKey as Secp256k1PubliKey, Secp256k1};

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
    match publik_key.type_url() {
        ED25519_TYPE_URL => {
            let public_key: Ed25519PublicKey = Ed25519PublicKey::from_bytes(&publik_key.to_bytes())
                .map_err(|e| Error::Other(format!("decode Ed25519PublicKey Error({})", e)))?;

            let signature: Signature = Signature::from_bytes(&signature_and_data.signature)
                .map_err(|e| Error::Other(format!("decode Sinature Failed({})", e)))?;
            public_key
                .verify(&sign_bytes, &signature)
                .map_err(|e| Error::Other(format!("ed25519 verify failed: ({})", e)))
        }
        SECP256K1_TYPE_URL => {
            // ref: https://docs.rs/secp256k1/latest/secp256k1/
            let secp = Secp256k1::verification_only();
            let public_key = Secp256k1PubliKey::from_slice(&publik_key.to_bytes())
                .map_err(|e| Error::Other(format!("Decode Secp256k1 error({})", e)))?;
            let message = Message::from_slice(&sign_bytes)
                .map_err(|e| Error::Other(format!("Decode Message Error({})", e)))?;
            let sig = ecdsa::Signature::from_compact(&signature_and_data.signature)
                .map_err(|e| Error::Other(format!("Decode scdsa Signature failed({})", e)))?;
            secp.verify_ecdsa(&message, &sig, &public_key)
                .map_err(|e| Error::Other(format!("verify ecdsa failed({})", e)))
        }
        _ => Err(Error::Other("No Support Crypto type".into())),
    }
}
