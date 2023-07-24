use crate::cosmos::error::Error;
use ibc_proto::cosmos::crypto::multisig::v1beta1::MultiSignature as RawMultiSignature;
use ibc_proto::google::protobuf::Any;
use prost::Message;

pub const MULTI_SIGNATURE_TYPE_URL: &str = "/cosmos.crypto.multisig.v1beta1.MultiSignature";

/// MultiSignature wraps the signatures from a multisig.LegacyAminoPubKey.
/// See cosmos.tx.v1betata1.ModeInfo.Multi for how to specify which signers
/// signed and with which modes.
#[derive(Clone, PartialEq)]
pub struct MultiSignature {
    pub signatures: Vec<Vec<u8>>,
}

impl From<RawMultiSignature> for MultiSignature {
    fn from(raw_multi_signature: RawMultiSignature) -> MultiSignature {
        MultiSignature {
            signatures: raw_multi_signature.signatures,
        }
    }
}

impl From<MultiSignature> for Any {
    fn from(multi_signature: MultiSignature) -> Any {
        let proto = RawMultiSignature {
            signatures: multi_signature.signatures,
        };

        Any {
            type_url: MULTI_SIGNATURE_TYPE_URL.to_owned(),
            value: proto.encode_to_vec(),
        }
    }
}

impl TryFrom<Any> for MultiSignature {
    type Error = Error;

    fn try_from(any: Any) -> Result<MultiSignature, Self::Error> {
        MultiSignature::try_from(&any)
    }
}

impl TryFrom<&Any> for MultiSignature {
    type Error = Error;

    fn try_from(any: &Any) -> Result<Self, Self::Error> {
        if any.type_url != MULTI_SIGNATURE_TYPE_URL {
            return Err(Error::Crypto);
        }

        let proto = RawMultiSignature::decode(&*any.value).map_err(|e| Error::Other {
            description: format!("{}", e),
        })?;

        Ok(proto.into())
    }
}
