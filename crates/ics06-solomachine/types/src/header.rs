//! Defines the domain type for solomachine headers

use crate::cosmos::crypto::PublicKey;
use crate::error::Error;
use alloc::string::ToString;
use bytes::Buf;
use core::fmt::{Display, Error as FmtError, Formatter};
use ibc_core::client::types::error::ClientError;
use ibc_core::primitives::prelude::*;
use ibc_core::primitives::Timestamp;
use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::lightclients::solomachine::v3::Header as RawSmHeader;
use ibc_proto::Protobuf;
use prost::Message;

pub const SOLOMACHINE_HEADER_TYPE_URL: &str = "/ibc.lightclients.solomachine.v3.Header";

/// Header defines a solo machine consensus header
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, PartialEq)]
pub struct Header {
    pub timestamp: Timestamp,
    pub signature: Vec<u8>,
    pub new_public_key: PublicKey,
    pub new_diversifier: String,
}

impl core::fmt::Debug for Header {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        write!(f, " Header {{...}}")
    }
}

impl Display for Header {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        write!(
            f,
            "Header {{ timestamp: {}, signature: {:?}, new_public_key: {:?}, new_diversifier: {} }}",
            self.timestamp,
            self.signature,
            self.new_public_key,
            self.new_diversifier
        )
    }
}

impl Protobuf<RawSmHeader> for Header {}

impl TryFrom<RawSmHeader> for Header {
    type Error = Error;

    fn try_from(raw: RawSmHeader) -> Result<Self, Self::Error> {
        let timestamp =
            Timestamp::from_nanoseconds(raw.timestamp).map_err(Error::ParseTimeError)?;
        let signature = raw.signature;

        let new_public_key =
            PublicKey::try_from(raw.new_public_key.ok_or(Error::PublicKeyIsEmpty)?)
                .map_err(Error::PublicKeyParseFailed)?;
        let new_diversifier = raw.new_diversifier;
        Ok(Self {
            timestamp,
            signature,
            new_public_key,
            new_diversifier,
        })
    }
}

impl From<Header> for RawSmHeader {
    fn from(value: Header) -> Self {
        Self {
            timestamp: value.timestamp.nanoseconds(),
            signature: value.signature,
            new_public_key: Some(value.new_public_key.to_any()),
            new_diversifier: value.new_diversifier,
        }
    }
}

impl Protobuf<Any> for Header {}

impl TryFrom<Any> for Header {
    type Error = ClientError;

    fn try_from(raw: Any) -> Result<Self, Self::Error> {
        use core::ops::Deref;

        match raw.type_url.as_str() {
            SOLOMACHINE_HEADER_TYPE_URL => decode_header(raw.value.deref()).map_err(Into::into),
            _ => Err(ClientError::UnknownHeaderType {
                header_type: raw.type_url,
            }),
        }
    }
}

impl From<Header> for Any {
    fn from(header: Header) -> Self {
        Any {
            type_url: SOLOMACHINE_HEADER_TYPE_URL.to_string(),
            value: Protobuf::<RawSmHeader>::encode_vec(header),
        }
    }
}

pub fn decode_header<B: Buf>(buf: B) -> Result<Header, Error> {
    RawSmHeader::decode(buf).map_err(Error::Decode)?.try_into()
}

#[test]
fn test_header_der_ser() {
    const EXAMPLE_JSON: &str = "{\"@type\":\"/cosmos.crypto.ed25519.PubKey\",\"key\":\"sEEsVGkXvyewKLWMJbHVDRkBoerW0IIwmj1rHkabtHU=\"}";

    let fix_public_key = EXAMPLE_JSON.parse::<PublicKey>().unwrap();
    let temp_header = Header {
        timestamp: Timestamp::now(),
        signature: vec![1, 2, 3],
        new_public_key: fix_public_key,
        new_diversifier: "test".into(),
    };
    let any_header = Any::from(temp_header);
    let encode_any_header = any_header.encode_to_vec();
    let decode_any_header = Any::decode(encode_any_header.as_ref()).unwrap();
    println!("decode_any_header = {:?}", decode_any_header);
    assert_eq!(decode_any_header, any_header);
}
