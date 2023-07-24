//! Legacy Amino support.

use super::PublicKey;
use crate::cosmos::error::Error;
use crate::prelude::*;
use ibc_proto::cosmos::crypto::multisig::LegacyAminoPubKey as RawLegacyAminoPubKey;
use ibc_proto::google::protobuf::Any;
use prost::Message;

/// Protobuf [`Any`] type URL for [`LegacyAminoMultisig`].
pub const LEGACY_AMINO_MULTISIG_TYPE_URL: &str = "/cosmos.crypto.multisig.LegacyAminoPubKey";

/// Legacy Amino multisig key.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LegacyAminoMultisig {
    /// Multisig threshold.
    pub threshold: u32,

    /// Public keys which comprise the multisig key.
    pub public_keys: Vec<PublicKey>,
}

impl From<LegacyAminoMultisig> for Any {
    fn from(amino_multisig: LegacyAminoMultisig) -> Any {
        let proto = RawLegacyAminoPubKey {
            threshold: amino_multisig.threshold,
            public_keys: amino_multisig
                .public_keys
                .into_iter()
                .map(|pk| pk.into())
                .collect(),
        };

        Any {
            type_url: LEGACY_AMINO_MULTISIG_TYPE_URL.to_owned(),
            value: proto.encode_to_vec(),
        }
    }
}

impl TryFrom<Any> for LegacyAminoMultisig {
    type Error = Error;

    fn try_from(any: Any) -> Result<LegacyAminoMultisig, Self::Error> {
        LegacyAminoMultisig::try_from(&any)
    }
}

impl TryFrom<&Any> for LegacyAminoMultisig {
    type Error = Error;

    fn try_from(any: &Any) -> Result<Self, Self::Error> {
        if any.type_url != LEGACY_AMINO_MULTISIG_TYPE_URL {
            return Err(Error::Crypto);
        }

        let proto = RawLegacyAminoPubKey::decode(&*any.value).map_err(|e| Error::Other {
            description: format!("{}", e),
        })?;
        let public_keys = proto
            .public_keys
            .into_iter()
            .map(PublicKey::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            threshold: proto.threshold,
            public_keys,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::LegacyAminoMultisig;
    use crate::alloc::borrow::ToOwned;
    use crate::cosmos::crypto::public_key::SECP256K1_TYPE_URL;
    use hex_literal::hex;
    use ibc_proto::google::protobuf::Any;

    #[test]
    fn any_round_trip() {
        let any = Any {
            type_url: "/cosmos.crypto.multisig.LegacyAminoPubKey".to_owned(),
            value: hex!("080312460a1f2f636f736d6f732e63727970746f2e736563703235366b312e5075624b657912230a210316eb99be27392e258ded83dc1378e507acf1bb726fa407167e709461b3a631cb12460a1f2f636f736d6f732e63727970746f2e736563703235366b312e5075624b657912230a210363deebf13d30a9840f275d01911f3e05f3fb5f88554f52b2ef534dce06b1da5912460a1f2f636f736d6f732e63727970746f2e736563703235366b312e5075624b657912230a21032e253cf8214f3d466ed296b9919821ae6681806c91b3c2063a45a8b85ce7e11512460a1f2f636f736d6f732e63727970746f2e736563703235366b312e5075624b657912230a210326ffd12bd115f260a371f2f09bf29286e4c9681c7bc109f4604c82ed82d6d23212460a1f2f636f736d6f732e63727970746f2e736563703235366b312e5075624b657912230a210343a3b485021493370286c9f4725358a3fd459576f963dcc158cb82c02276b67f").into(),
        };

        let pk = LegacyAminoMultisig::try_from(&any).unwrap();
        assert_eq!(pk.threshold, 3);
        assert_eq!(pk.public_keys.len(), 5);
        assert_eq!(pk.public_keys[0].type_url(), SECP256K1_TYPE_URL);
        assert_eq!(
            pk.public_keys[0],
            tendermint::PublicKey::from_raw_secp256k1(&hex!(
                "0316eb99be27392e258ded83dc1378e507acf1bb726fa407167e709461b3a631cb"
            ))
            .unwrap()
            .into()
        );

        // Ensure serialized key round trips
        assert_eq!(any, Any::from(pk));
    }
}
