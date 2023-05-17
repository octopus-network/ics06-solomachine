use crate::prelude::*;
use ibc::core::ics03_connection::error::ConnectionError;
use ibc::core::ics04_channel::error::ChannelError;

use displaydoc::Display;
use ibc::core::ics02_client::error::ClientError;
use ibc::core::timestamp::ParseTimestampError;

#[derive(Debug, Display)]
pub enum Error {
    /// dummy error
    Dummy,
    /// decode error: `{0}`
    Decode(prost::DecodeError),
    /// consensus state PublicKey is None
    EmptyConsensusStatePublicKey,
    /// invlid height
    InvalidHeight(ClientError),
    /// invalid raw client id: `{client_id}`
    InvalidRawClientId { client_id: String },
    /// unknow data type: `{0}`
    UnknownDataType(i32),
    /// prase time error
    ParseTimeError(ParseTimestampError),
    /// Channel error: `{0}`
    ChannelError(ChannelError),
    /// Client error: `{0}`
    ClientError(ClientError),
    /// Connection error: `{0}`
    ConnectionError(ConnectionError),
    /// timestamp cannot be 0
    TimeStampIsEmpty,
    /// diversifier cannot contain only spaces
    DriversifierContainOnlySpaces,
    /// public key parsed failed: `{0}`
    PublicKeyParseFailed(eyre::ErrReport),
    /// public key is empty
    PublicKeyIsEmpty,
    /// consensus state is empty
    ConsensusStateIsEmpty,
    /// SignatureAndData empty
    SignatureAndDataIsEmpty,
    /// Sequence cannot be zero
    SequenceCannotZero,
    /// Proof cannot be empty
    ProofCannotEmpty,
    /// ChannelEnd is empty
    ChannelEndIsEmpty,
    /// ClientState is empty
    ClientStateIsEmpty,
    /// ConnectionEnd is empty
    ConnectionEndIsEmpty,
    /// Other : `{0}`
    Other(String),
}

impl From<Error> for ClientError {
    fn from(e: Error) -> Self {
        Self::ClientSpecific {
            description: e.to_string(),
        }
    }
}

pub(crate) trait IntoResult<T, E> {
    fn into_result(self) -> Result<T, E>;
}
