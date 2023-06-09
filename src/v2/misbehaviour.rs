use crate::prelude::*;
use crate::v2::error::Error;
use crate::v2::proof::types::signature_and_data::SignatureAndData;
use bytes::Buf;
use ibc::core::ics02_client::error::ClientError;
use ibc::core::ics24_host::identifier::ClientId;
use ibc::Height;
use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::lightclients::solomachine::v2::Misbehaviour as RawSmMisbehaviour;
use ibc_proto::protobuf::Protobuf;
use prost::Message;

pub const SOLOMACHINE_MISBEHAVIOUR_TYPE_URL: &str = "/ibc.lightclients.solomachine.v2.Misbehaviour";

/// Misbehaviour defines misbehaviour for a solo machine which consists
/// of a sequence and two signatures over different messages at that sequence.
#[derive(Clone, PartialEq)]
pub struct Misbehaviour {
    pub client_id: ClientId,
    pub sequence: Height,
    pub signature_one: SignatureAndData,
    pub signature_two: SignatureAndData,
}

impl Protobuf<RawSmMisbehaviour> for Misbehaviour {}

impl TryFrom<RawSmMisbehaviour> for Misbehaviour {
    type Error = Error;

    fn try_from(raw: RawSmMisbehaviour) -> Result<Self, Self::Error> {
        let client_id: ClientId = raw
            .client_id
            .parse()
            .map_err(|_| Error::InvalidRawClientId {
                client_id: raw.client_id.clone(),
            })?;
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
            client_id,
            sequence,
            signature_one,
            signature_two,
        })
    }
}

impl From<Misbehaviour> for RawSmMisbehaviour {
    fn from(value: Misbehaviour) -> Self {
        let sequence = value.sequence.revision_height();
        let client_id = value.client_id.to_string();
        Self {
            client_id,
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
            value: Protobuf::<RawSmMisbehaviour>::encode_vec(&misbehaviour),
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
            "ClientId({}), Sequence({}), SignatureOne({}), SignatureTwo({})",
            self.client_id, self.sequence, self.signature_two, self.signature_two
        )
    }
}
