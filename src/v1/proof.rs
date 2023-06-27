// // use crate::cosmos::crypto::PublicKey;
// use crate::prelude::*;
// use crate::v1::client_state::PublicKey;
// use crate::v1::error::Error;
// use crate::v1::header::Header;
// use crate::v1::proof::types::channel_state_data::ChannelStateData;
// use crate::v1::proof::types::client_state_data::ClientStateData;
// use crate::v1::proof::types::connection_state_data::ConnectionStateData;
// use crate::v1::proof::types::consensus_state_data::ConsensusStateData;
// use crate::v1::proof::types::header_data::HeaderData;
// use crate::v1::proof::types::next_sequence_recv_data::NextSequenceRecvData;
// use crate::v1::proof::types::packet_acknowledgement_data::PacketAcknowledgementData;
// use crate::v1::proof::types::packet_commitment_data::PacketCommitmentData;
// use crate::v1::proof::types::packet_receipt_absence_data::PacketReceiptAbsenceData;
// use crate::v1::proof::types::sign_bytes::SignBytes;
// use crate::v1::proof::types::signature_and_data::SignatureAndData;
// use crate::v1::proof::types::DataType;
// use ibc_proto::ibc::core::commitment::v1::MerklePath;
// use ibc_proto::protobuf::Protobuf;
// use prost::Message;
// use tendermint::crypto::signature::Verifier;
// use tendermint::Signature;

// pub mod types;

// // Verify_signature verifies if the the provided public key generated the signature
// // over the given data. Single and Multi signature public keys are supported.
// // The signature data type must correspond to the public key type. An error is
// // returned if signature verification fails or an invalid SignatureData type is
// // provided.
// // todo (davirain) ref: https://github.com/cosmos/ibc-go/blob/6f1d8d672705c6e8f5b74a396d883e2834a6b943/modules/light-clients/06-solomachine/types/proof.go#L22
// pub fn verify_signature(
//     publik_key: PublicKey,
//     sign_bytes: Vec<u8>,
//     signature_and_data: SignatureAndData,
// ) -> Result<(), Error> {
//     let signature = Signature::try_from(signature_and_data.signature)
//         .map_err(|e| Error::Other(format!("{}", e)))?;
//     tendermint::crypto::default::signature::Verifier::verify(
//         publik_key.into(),
//         &sign_bytes,
//         &signature,
//     )
//     .map_err(|e| Error::Other(format!("{}", e)))
// }

// // MisbehaviourSignBytes returns the sign bytes for verification of misbehaviour.
// pub fn misbehaviour_sign_bytes(
//     sequence: u64,
//     timestamp: u64,
//     diversifier: String,
//     data_type: DataType,
//     data: Vec<u8>,
// ) -> Vec<u8> {
//     let sign_bytest = SignBytes {
//         sequence,
//         timestamp,
//         diversifier,
//         data_type,
//         data,
//     };

//     sign_bytest.encode_vec()
// }

// // HeaderSignBytes returns the sign bytes for verification of misbehaviour.
// pub fn header_sign_bytes(heder: Header) -> Vec<u8> {
//     let data = HeaderData {
//         new_pub_key: heder.new_public_key,
//         new_diversifier: heder.new_diversifier.clone(),
//     };
//     let data_bz = data.encode_vec();

//     let sign_bytes = SignBytes {
//         sequence: heder.sequence.revision_height(),
//         timestamp: heder.timestamp.nanoseconds(),
//         diversifier: heder.new_diversifier,
//         data_type: DataType::Header,
//         data: data_bz,
//     };
//     sign_bytes.encode_vec()
// }

// // ClientStateSignBytes returns the sign bytes for verification of the
// // client state.
// pub fn client_state_sign_bytes(
//     sequence: u64,
//     timestamp: u64,
//     diversifier: String,
//     path: MerklePath,
//     client_state: Vec<u8>,
// ) -> Vec<u8> {
//     let data_bz = client_state_data_bytes(path, client_state);
//     let sign_bytes = SignBytes {
//         sequence,
//         timestamp,
//         diversifier,
//         data_type: DataType::ClientState,
//         data: data_bz,
//     };
//     sign_bytes.encode_vec()
// }

// // ClientStateDataBytes returns the client state data bytes used in constructing
// // SignBytes.
// pub fn client_state_data_bytes(path: MerklePath, client_state: Vec<u8>) -> Vec<u8> {
//     let data = ClientStateData {
//         path: path.encode_to_vec(),
//         client_state,
//     };
//     data.encode_vec()
// }

// // ConsensusStateSignBytes returns the sign bytes for verification of the
// // consensus state.
// pub fn consensus_state_sign_bytes(
//     sequence: u64,
//     timestamp: u64,
//     diversifier: String,
//     path: MerklePath,
//     consensus_state: Vec<u8>,
// ) -> Vec<u8> {
//     let data_bz = consensus_state_data_bytes(path, consensus_state);
//     let sign_bytes = SignBytes {
//         sequence,
//         timestamp,
//         diversifier,
//         data_type: DataType::ConsensusState,
//         data: data_bz,
//     };
//     sign_bytes.encode_vec()
// }

// // ConsensusStateDataBytes returns the consensus state data bytes used in constructing
// // SignBytes.
// pub fn consensus_state_data_bytes(path: MerklePath, consensus_state: Vec<u8>) -> Vec<u8> {
//     let data = ConsensusStateData {
//         path: path.encode_to_vec(),
//         consensus_state,
//     };
//     data.encode_vec()
// }

// // ConnectionStateSignBytes returns the sign bytes for verification of the
// // connection state.
// pub fn connection_state_sign_bytes(
//     sequence: u64,
//     timestamp: u64,
//     diversifier: String,
//     path: MerklePath,
//     connection_end: Vec<u8>,
// ) -> Vec<u8> {
//     let data_bz = connection_state_data_bytes(path, connection_end);
//     let sign_bytes = SignBytes {
//         sequence,
//         timestamp,
//         diversifier,
//         data_type: DataType::ConnectionState,
//         data: data_bz,
//     };
//     sign_bytes.encode_vec()
// }

// // ConnectionStateDataBytes returns the connection state data bytes used in constructing
// // SignBytes.
// pub fn connection_state_data_bytes(path: MerklePath, connection_end: Vec<u8>) -> Vec<u8> {
//     let data = ConnectionStateData {
//         path: path.encode_to_vec(),
//         connection: connection_end,
//     };
//     data.encode_vec()
// }

// // ChannelStateSignBytes returns the sign bytes for verification of the
// // channel state.
// pub fn channel_state_sign_bytes(
//     sequence: u64,
//     timestamp: u64,
//     diversifier: String,
//     path: MerklePath,
//     channel_end: Vec<u8>,
// ) -> Vec<u8> {
//     let data_bz = channel_state_data_bytes(path, channel_end);
//     let sign_bytes = SignBytes {
//         sequence,
//         timestamp,
//         diversifier,
//         data_type: DataType::ChannelState,
//         data: data_bz,
//     };
//     sign_bytes.encode_vec()
// }

// // ChannelStateDataBytes returns the channel state data bytes used in constructing
// // SignBytes.
// pub fn channel_state_data_bytes(path: MerklePath, channel_end: Vec<u8>) -> Vec<u8> {
//     let data = ChannelStateData {
//         path: path.encode_to_vec(),
//         channel: channel_end,
//     };
//     data.encode_vec()
// }

// // PacketCommitmentSignBytes returns the sign bytes for verification of the
// // packet commitment.
// pub fn packet_commitment_sign_bytes(
//     sequence: u64,
//     timestamp: u64,
//     diversifier: String,
//     path: MerklePath,
//     commitment_bytes: Vec<u8>,
// ) -> Vec<u8> {
//     let data_bz = packet_commitment_data_bytes(path, commitment_bytes);
//     let sign_bytes = SignBytes {
//         sequence,
//         timestamp,
//         diversifier,
//         data_type: DataType::PacketCommitment,
//         data: data_bz,
//     };
//     sign_bytes.encode_vec()
// }

// // PacketCommitmentDataBytes returns the packet commitment data bytes used in constructing
// // SignBytes.
// pub fn packet_commitment_data_bytes(path: MerklePath, commitment_bytes: Vec<u8>) -> Vec<u8> {
//     let data = PacketCommitmentData {
//         path: path.encode_to_vec(),
//         commitment: commitment_bytes,
//     };
//     data.encode_vec()
// }

// // // PacketAcknowledgementSignBytes returns the sign bytes for verification of
// // // the acknowledgement.
// pub fn packet_acknowledgement_sign_bytes(
//     sequence: u64,
//     timestamp: u64,
//     diversifier: String,
//     path: MerklePath,
//     acknowledgement: Vec<u8>,
// ) -> Vec<u8> {
//     let data_bz = packet_acknowledgement_data_bytes(path, acknowledgement);
//     let sign_bytes = SignBytes {
//         sequence,
//         timestamp,
//         diversifier,
//         data_type: DataType::PacketAcknowledgement,
//         data: data_bz,
//     };
//     sign_bytes.encode_vec()
// }

// // PacketAcknowledgementDataBytes returns the packet acknowledgement data bytes used in constructing
// // SignBytes.
// pub fn packet_acknowledgement_data_bytes(path: MerklePath, acknowledgement: Vec<u8>) -> Vec<u8> {
//     let data = PacketAcknowledgementData {
//         path: path.encode_to_vec(),
//         acknowledgement,
//     };
//     data.encode_vec()
// }

// // PacketReceiptAbsenceSignBytes returns the sign bytes for verification
// // of the absence of an receipt.
// pub fn packet_receipt_absence_sign_bytes(
//     sequence: u64,
//     timestamp: u64,
//     diversifier: String,
//     path: MerklePath,
// ) -> Vec<u8> {
//     let data_bz = packet_receipt_absence_data_bytes(path);
//     let sign_bytes = SignBytes {
//         sequence,
//         timestamp,
//         diversifier,
//         data_type: DataType::PacketReceiptAbsence,
//         data: data_bz,
//     };
//     sign_bytes.encode_vec()
// }

// // PacketReceiptAbsenceDataBytes returns the packet receipt absence data bytes
// // used in constructing SignBytes.
// pub fn packet_receipt_absence_data_bytes(path: MerklePath) -> Vec<u8> {
//     let data = PacketReceiptAbsenceData {
//         path: path.encode_to_vec(),
//     };
//     data.encode_vec()
// }

// // NextSequenceRecvSignBytes returns the sign bytes for verification of the next
// // sequence to be received.
// pub fn next_sequence_recv_sign_bytes(
//     sequence: u64,
//     timestamp: u64,
//     diversifier: String,
//     path: MerklePath,
//     next_sequence_recv: Vec<u8>,
// ) -> Vec<u8> {
//     let data_bz = next_sequence_recv_data_bytes(path, next_sequence_recv);
//     let sign_bytes = SignBytes {
//         sequence,
//         timestamp,
//         diversifier,
//         data_type: DataType::NextSequenceRecv,
//         data: data_bz,
//     };
//     sign_bytes.encode_vec()
// }

// // NextSequenceRecvDataBytes returns the next sequence recv data bytes used in constructing
// // SignBytes.
// pub fn next_sequence_recv_data_bytes(path: MerklePath, next_sequence_recv: Vec<u8>) -> Vec<u8> {
//     let data = NextSequenceRecvData {
//         path: path.encode_to_vec(),
//         next_seq_recv: next_sequence_recv,
//     };
//     data.encode_vec()
// }
