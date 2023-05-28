use crate::error::Error;
use crate::prelude::*;
use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::lightclients::solomachine::v2::ClientStateData as RawClientStateData;
use ibc_proto::protobuf::Protobuf;

pub const SOLOMACHINE_CLIENT_STATE_DATA_TYPE_URL: &str =
    "/ibc.lightclients.solomachine.v1.ClientStateData";

/// ClientStateData returns the SignBytes data for client state verification.
#[derive(Clone, PartialEq)]
pub struct ClientStateData {
    pub path: Vec<u8>,
    // Ics06 solomachine client state
    pub client_state: Vec<u8>,
}

impl Protobuf<RawClientStateData> for ClientStateData {}

impl TryFrom<RawClientStateData> for ClientStateData {
    type Error = Error;

    fn try_from(raw: RawClientStateData) -> Result<Self, Self::Error> {
        Ok(Self {
            path: raw.path,
            client_state: raw.client_state.ok_or(Error::ClientStateIsEmpty)?.value,
        })
    }
}

impl From<ClientStateData> for RawClientStateData {
    fn from(value: ClientStateData) -> Self {
        Self {
            path: value.path,
            client_state: Some(Any {
                type_url: SOLOMACHINE_CLIENT_STATE_DATA_TYPE_URL.to_string(),
                value: value.client_state,
            }),
        }
    }
}
