use anyhow::Result;

pub mod compact_bit_array;
pub mod multisignature;

// PubKey defines a public key and extends proto.Message.
pub trait PubKeyInterface: prost::Message {
    fn address(&self) -> String;
    fn bytes(&self) -> Vec<u8>;
    fn verify_signature(&self, msg: &[u8], sig: &[u8]) -> bool;
    fn equals(&self, other: &Self) -> bool;
    fn type_(&self) -> String;
}

// LedgerPrivKey defines a private key that is not a proto message. For now,
// LedgerSecp256k1 keys are not converted to proto.Message yet, this is why
// they use LedgerPrivKey instead of PrivKey. All other keys must use PrivKey
// instead of LedgerPrivKey.
// TODO https://github.com/cosmos/cosmos-sdk/issues/7357.
pub trait LedgerPrivKeyInterface {
    fn bytes(&self) -> Vec<u8>;
    fn sign(&self, msg: &[u8]) -> Result<Vec<u8>>;
    fn pub_key<T: PubKeyInterface>(&self) -> T;
    fn equals(&self, other: &Self) -> bool;
    fn type_(&self) -> String;
}

// LedgerPrivKeyAminoJSON is a Ledger PrivKey type that supports signing with
// SIGN_MODE_LEGACY_AMINO_JSON. It is added as a non-breaking change, instead of directly
// on the LedgerPrivKey interface (whose Sign method will sign with TEXTUAL),
// and will be deprecated/removed once LEGACY_AMINO_JSON is removed.
pub trait LedgerPrivKeyAminoJSONInterface: LedgerPrivKeyInterface {
    // SignLedgerAminoJSON signs a messages on the Ledger device using
    // SIGN_MODE_LEGACY_AMINO_JSON.
    fn sign_ledger_amino_json(&self, msg: &[u8]) -> Result<Vec<u8>>;
}

// PrivKey defines a private key and extends proto.Message. For now, it extends
// LedgerPrivKey (see godoc for LedgerPrivKey). Ultimately, we should remove
// LedgerPrivKey and add its methods here directly.
// TODO https://github.com/cosmos/cosmos-sdk/issues/7357.
pub trait PrivKeyInterface: prost::Message + LedgerPrivKeyInterface {}
