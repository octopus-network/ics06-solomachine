//! Defines the misbehaviour type for the solomachine light client

use crate::error::Error;
use crate::proof::types::signature_and_data::SignatureAndData;
use bytes::Buf;
use ibc_core::client::types::error::ClientError;
use ibc_core::client::types::Height;
use ibc_core::primitives::prelude::*;
use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::lightclients::solomachine::v3::Misbehaviour as RawSmMisbehaviour;
use ibc_proto::Protobuf;
use prost::Message;

pub const SOLOMACHINE_MISBEHAVIOUR_TYPE_URL: &str = "/ibc.lightclients.solomachine.v3.Misbehaviour";

/// Misbehaviour defines misbehaviour for a solo machine which consists
/// of a sequence and two signatures over different messages at that sequence.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, PartialEq)]
pub struct Misbehaviour {
    /// The sequence number at which the infraction occurred
    pub sequence: Height,
    /// The first signature
    pub signature_one: SignatureAndData,
    /// The second signature
    pub signature_two: SignatureAndData,
}

impl Protobuf<RawSmMisbehaviour> for Misbehaviour {}

impl TryFrom<RawSmMisbehaviour> for Misbehaviour {
    type Error = Error;

    fn try_from(raw: RawSmMisbehaviour) -> Result<Self, Self::Error> {
        let sequence = Height::new(0, raw.sequence).map_err(Error::InvalidHeight)?;
        let signature_one: SignatureAndData = raw
            .signature_one
            .ok_or(Error::SignatureAndDataIsEmpty)?
            .try_into()?;
        let signature_two: SignatureAndData = raw
            .signature_two
            .ok_or(Error::SignatureAndDataIsEmpty)?
            .try_into()?;

        Ok(Self {
            sequence,
            signature_one,
            signature_two,
        })
    }
}

impl From<Misbehaviour> for RawSmMisbehaviour {
    fn from(value: Misbehaviour) -> Self {
        let sequence = value.sequence.revision_height();

        Self {
            sequence,
            signature_one: Some(value.signature_one.into()),
            signature_two: Some(value.signature_two.into()),
        }
    }
}

impl Protobuf<Any> for Misbehaviour {}

impl TryFrom<Any> for Misbehaviour {
    type Error = ClientError;

    fn try_from(raw: Any) -> Result<Self, ClientError> {
        use core::ops::Deref;

        fn decode_misbehaviour<B: Buf>(buf: B) -> Result<Misbehaviour, Error> {
            RawSmMisbehaviour::decode(buf)
                .map_err(Error::Decode)?
                .try_into()
        }

        match raw.type_url.as_str() {
            SOLOMACHINE_MISBEHAVIOUR_TYPE_URL => {
                decode_misbehaviour(raw.value.deref()).map_err(Into::into)
            }
            _ => Err(ClientError::UnknownMisbehaviourType {
                misbehaviour_type: raw.type_url,
            }),
        }
    }
}

impl From<Misbehaviour> for Any {
    fn from(misbehaviour: Misbehaviour) -> Self {
        Any {
            type_url: SOLOMACHINE_MISBEHAVIOUR_TYPE_URL.to_string(),
            value: Protobuf::<RawSmMisbehaviour>::encode_vec(misbehaviour),
        }
    }
}

pub fn decode_misbehaviour<B: Buf>(buf: B) -> Result<Misbehaviour, Error> {
    RawSmMisbehaviour::decode(buf)
        .map_err(Error::Decode)?
        .try_into()
}

impl core::fmt::Display for Misbehaviour {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        write!(
            f,
            "Sequence({}), SignatureOne({}), SignatureTwo({})",
            self.sequence, self.signature_two, self.signature_two
        )
    }
}
