use crate::client_state::ClientState;
use crate::consensus_state::ConsensusState;
use crate::cosmos::crypto::PublicKey;
use crate::error::Error;
use crate::header::Header;
use crate::proof::types::channel_state_data::ChannelStateData;
use crate::proof::types::client_state_data::ClientStateData;
use crate::proof::types::connection_state_data::ConnectionStateData;
use crate::proof::types::consensus_state_data::ConsensusStateData;
use crate::proof::types::header_data::HeaderData;
use crate::proof::types::next_sequence_recv_data::NextSequenceRecvData;
use crate::proof::types::packet_acknowledgement_data::PacketAcknowledgementData;
use crate::proof::types::packet_commitment_data::PacketCommitmentData;
use crate::proof::types::packet_receipt_absence_data::PacketReceiptAbsenceData;
use crate::proof::types::sign_bytes::SignBytes;
use crate::proof::types::DataType;
use crate::signature_and_data::SignatureAndData;
use alloc::string::String;
use alloc::vec::Vec;
use ibc::core::ics03_connection::connection::ConnectionEnd;
use ibc::core::ics04_channel::channel::ChannelEnd;
use ibc::core::ics24_host::path::Path;
use ibc_proto::protobuf::Protobuf;

pub mod types;

// Verify_signature verifies if the the provided public key generated the signature
// over the given data. Single and Multi signature public keys are supported.
// The signature data type must correspond to the public key type. An error is
// returned if signature verification fails or an invalid SignatureData type is
// provided.
pub fn verify_signature(
    _publik_key: PublicKey,
    _sign_bytes: Vec<u8>,
    _signature_and_data: SignatureAndData,
) -> Result<(), Error> {
    todo!()
}

// MisbehaviourSignBytes returns the sign bytes for verification of misbehaviour.
pub fn misbehaviour_sign_bytes(
    sequence: u64,
    timestamp: u64,
    diversifier: String,
    data_type: DataType,
    data: Vec<u8>,
) -> Vec<u8> {
    let sign_bytest = SignBytes {
        sequence,
        timestamp,
        diversifier,
        data_type,
        data,
    };

    sign_bytest.encode_vec()
}

// HeaderSignBytes returns the sign bytes for verification of misbehaviour.
pub fn header_sign_bytes(heder: Header) -> Vec<u8> {
    let data = HeaderData {
        new_pub_key: heder.new_public_key,
        new_diversifier: heder.new_diversifier.clone(),
    };
    let data_bz = data.encode_vec();

    let sign_bytes = SignBytes {
        sequence: heder.sequence.revision_height(),
        timestamp: heder.timestamp.nanoseconds(),
        diversifier: heder.new_diversifier,
        data_type: DataType::Header,
        data: data_bz,
    };
    sign_bytes.encode_vec()
}

// ClientStateSignBytes returns the sign bytes for verification of the
// client state.
pub fn client_state_sign_bytes(
    sequence: u64,
    timestamp: u64,
    diversifier: String,
    path: Path,
    client_state: ClientState,
) -> Vec<u8> {
    let data_bz = client_state_data_bytes(path, client_state);
    let sign_bytes = SignBytes {
        sequence,
        timestamp,
        diversifier,
        data_type: DataType::ClientState,
        data: data_bz,
    };
    sign_bytes.encode_vec()
}

// ClientStateDataBytes returns the client state data bytes used in constructing
// SignBytes.
pub fn client_state_data_bytes(path: Path, client_state: ClientState) -> Vec<u8> {
    let data = ClientStateData {
        path: alloc::format!("{}", path).as_bytes().to_vec(),
        client_state,
    };
    data.encode_vec()
}

// ConsensusStateSignBytes returns the sign bytes for verification of the
// consensus state.
pub fn consensus_state_sign_bytes(
    sequence: u64,
    timestamp: u64,
    diversifier: String,
    path: Path,
    consensus_state: ConsensusState,
) -> Vec<u8> {
    let data_bz = consensus_state_data_bytes(path, consensus_state);
    let sign_bytes = SignBytes {
        sequence,
        timestamp,
        diversifier,
        data_type: DataType::ConsensusState,
        data: data_bz,
    };
    sign_bytes.encode_vec()
}

// ConsensusStateDataBytes returns the consensus state data bytes used in constructing
// SignBytes.
pub fn consensus_state_data_bytes(path: Path, consensus_state: ConsensusState) -> Vec<u8> {
    let data = ConsensusStateData {
        path: alloc::format!("{}", path).as_bytes().to_vec(),
        consensus_state,
    };
    data.encode_vec()
}

// ConnectionStateSignBytes returns the sign bytes for verification of the
// connection state.
pub fn connection_state_sign_bytes(
    sequence: u64,
    timestamp: u64,
    diversifier: String,
    path: Path,
    connection_end: ConnectionEnd,
) -> Vec<u8> {
    let data_bz = connection_state_data_bytes(path, connection_end);
    let sign_bytes = SignBytes {
        sequence,
        timestamp,
        diversifier,
        data_type: DataType::ConnectionState,
        data: data_bz,
    };
    sign_bytes.encode_vec()
}

// ConnectionStateDataBytes returns the connection state data bytes used in constructing
// SignBytes.
pub fn connection_state_data_bytes(path: Path, connection_end: ConnectionEnd) -> Vec<u8> {
    let data = ConnectionStateData {
        path: alloc::format!("{}", path).as_bytes().to_vec(),
        connection: connection_end,
    };
    data.encode_vec()
}

// ChannelStateSignBytes returns the sign bytes for verification of the
// channel state.
pub fn channel_state_sign_bytes(
    sequence: u64,
    timestamp: u64,
    diversifier: String,
    path: Path,
    channel_end: ChannelEnd,
) -> Vec<u8> {
    let data_bz = channel_state_data_bytes(path, channel_end);
    let sign_bytes = SignBytes {
        sequence,
        timestamp,
        diversifier,
        data_type: DataType::ChannelState,
        data: data_bz,
    };
    sign_bytes.encode_vec()
}

// ChannelStateDataBytes returns the channel state data bytes used in constructing
// SignBytes.
pub fn channel_state_data_bytes(path: Path, channel_end: ChannelEnd) -> Vec<u8> {
    let data = ChannelStateData {
        path: alloc::format!("{}", path).as_bytes().to_vec(),
        channel: channel_end,
    };
    data.encode_vec()
}

// PacketCommitmentSignBytes returns the sign bytes for verification of the
// packet commitment.
pub fn packet_commitment_sign_bytes(
    sequence: u64,
    timestamp: u64,
    diversifier: String,
    path: Path,
    commitment_bytes: Vec<u8>,
) -> Vec<u8> {
    let data_bz = packet_commitment_data_bytes(path, commitment_bytes);
    let sign_bytes = SignBytes {
        sequence,
        timestamp,
        diversifier,
        data_type: DataType::PacketCommitment,
        data: data_bz,
    };
    sign_bytes.encode_vec()
}

// PacketCommitmentDataBytes returns the packet commitment data bytes used in constructing
// SignBytes.
pub fn packet_commitment_data_bytes(path: Path, commitment_bytes: Vec<u8>) -> Vec<u8> {
    let data = PacketCommitmentData {
        path: alloc::format!("{}", path).as_bytes().to_vec(),
        commitment: commitment_bytes,
    };
    data.encode_vec()
}

// // PacketAcknowledgementSignBytes returns the sign bytes for verification of
// // the acknowledgement.
pub fn packet_acknowledgement_sign_bytes(
    sequence: u64,
    timestamp: u64,
    diversifier: String,
    path: Path,
    acknowledgement: Vec<u8>,
) -> Vec<u8> {
    let data_bz = packet_acknowledgement_data_bytes(path, acknowledgement);
    let sign_bytes = SignBytes {
        sequence,
        timestamp,
        diversifier,
        data_type: DataType::PacketAcknowledgement,
        data: data_bz,
    };
    sign_bytes.encode_vec()
}

// PacketAcknowledgementDataBytes returns the packet acknowledgement data bytes used in constructing
// SignBytes.
pub fn packet_acknowledgement_data_bytes(path: Path, acknowledgement: Vec<u8>) -> Vec<u8> {
    let data = PacketAcknowledgementData {
        path: alloc::format!("{}", path).as_bytes().to_vec(),
        acknowledgement,
    };
    data.encode_vec()
}

// PacketReceiptAbsenceSignBytes returns the sign bytes for verification
// of the absence of an receipt.
pub fn packet_receipt_absence_sign_bytes(
    sequence: u64,
    timestamp: u64,
    diversifier: String,
    path: Path,
) -> Vec<u8> {
    let data_bz = packet_receipt_absence_data_bytes(path);
    let sign_bytes = SignBytes {
        sequence,
        timestamp,
        diversifier,
        data_type: DataType::PacketReceiptAbsence,
        data: data_bz,
    };
    sign_bytes.encode_vec()
}

// PacketReceiptAbsenceDataBytes returns the packet receipt absence data bytes
// used in constructing SignBytes.
pub fn packet_receipt_absence_data_bytes(path: Path) -> Vec<u8> {
    let data = PacketReceiptAbsenceData {
        path: alloc::format!("{}", path).as_bytes().to_vec(),
    };
    data.encode_vec()
}

// NextSequenceRecvSignBytes returns the sign bytes for verification of the next
// sequence to be received.
pub fn next_sequence_recv_sign_bytes(
    sequence: u64,
    timestamp: u64,
    diversifier: String,
    path: Path,
    next_sequence_recv: u64,
) -> Vec<u8> {
    let data_bz = next_sequence_recv_data_bytes(path, next_sequence_recv);
    let sign_bytes = SignBytes {
        sequence,
        timestamp,
        diversifier,
        data_type: DataType::NextSequenceRecv,
        data: data_bz,
    };
    sign_bytes.encode_vec()
}

// NextSequenceRecvDataBytes returns the next sequence recv data bytes used in constructing
// SignBytes.
pub fn next_sequence_recv_data_bytes(path: Path, next_sequence_recv: u64) -> Vec<u8> {
    let data = NextSequenceRecvData {
        path: alloc::format!("{}", path).as_bytes().to_vec(),
        next_seq_recv: next_sequence_recv,
    };
    data.encode_vec()
}
