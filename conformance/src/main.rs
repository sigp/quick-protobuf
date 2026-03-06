#[allow(dead_code, unused_variables)]
mod generated;

use std::borrow::Cow;
use std::io::{self, Read, Write};

use quick_protobuf::{BytesReader, MessageRead, MessageWrite, Writer};

use generated::conformance::mod_ConformanceRequest::OneOfpayload;
use generated::conformance::mod_ConformanceResponse::OneOfresult;
use generated::conformance::{ConformanceRequest, ConformanceResponse, WireFormat};
use generated::protobuf_test_messages::proto2::TestAllTypesProto2;
use generated::protobuf_test_messages::proto3::TestAllTypesProto3;

fn main() {
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();

    loop {
        // Read 4-byte little-endian length prefix
        let mut len_buf = [0u8; 4];
        match stdin.read_exact(&mut len_buf) {
            Ok(()) => {}
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => break,
            Err(e) => {
                eprintln!("error reading length: {e}");
                std::process::exit(1);
            }
        }

        let msg_len = u32::from_le_bytes(len_buf) as usize;
        if msg_len == 0 {
            break;
        }

        // Read the request message
        let mut request_buf = vec![0u8; msg_len];
        if let Err(e) = stdin.read_exact(&mut request_buf) {
            eprintln!("error reading request: {e}");
            std::process::exit(1);
        }

        let response = handle_request(&request_buf);

        // Write response
        let size = response.get_size();
        let mut out_buf = vec![0u8; size];
        let mut writer = Writer::new(&mut out_buf[..]);
        response
            .write_message(&mut writer)
            .expect("failed to serialize response");

        let len_bytes = (size as u32).to_le_bytes();
        stdout
            .write_all(&len_bytes)
            .expect("failed to write length");
        stdout
            .write_all(&out_buf)
            .expect("failed to write response");
        stdout.flush().expect("failed to flush stdout");
    }
}

fn handle_request(request_buf: &[u8]) -> ConformanceResponse<'static> {
    let mut reader = BytesReader::from_bytes(request_buf);
    let request = match ConformanceRequest::from_reader(&mut reader, request_buf) {
        Ok(req) => req,
        Err(e) => {
            return make_error(format!("failed to parse ConformanceRequest: {e}"));
        }
    };

    // Extract owned values from request before consuming payload
    let output_format = request
        .requested_output_format
        .unwrap_or(WireFormat::UNSPECIFIED);
    let message_type = request
        .message_type
        .map(|s| s.into_owned())
        .unwrap_or_default();
    let payload = request.payload;

    // Only handle protobuf input — extract owned bytes
    let payload_bytes = match payload {
        OneOfpayload::protobuf_payload(bytes) => bytes.into_owned(),
        OneOfpayload::json_payload(_) => {
            return make_skipped("JSON input is not supported");
        }
        OneOfpayload::jspb_payload(_) => {
            return make_skipped("JSPB input is not supported");
        }
        OneOfpayload::text_payload(_) => {
            return make_skipped("Text format input is not supported");
        }
        OneOfpayload::None => {
            return make_error("no payload in ConformanceRequest".to_string());
        }
    };

    // Only handle protobuf output
    match output_format {
        WireFormat::PROTOBUF => {}
        WireFormat::JSON => {
            return make_skipped("JSON output is not supported");
        }
        WireFormat::JSPB => {
            return make_skipped("JSPB output is not supported");
        }
        WireFormat::TEXT_FORMAT => {
            return make_skipped("Text format output is not supported");
        }
        WireFormat::UNSPECIFIED => {
            return make_error("unspecified output format".to_string());
        }
    }

    // Determine message type and round-trip
    match message_type.as_str() {
        "protobuf_test_messages.proto2.TestAllTypesProto2" => {
            round_trip::<TestAllTypesProto2>(&payload_bytes)
        }
        "protobuf_test_messages.proto3.TestAllTypesProto3" => {
            round_trip::<TestAllTypesProto3>(&payload_bytes)
        }
        other => make_skipped(&format!("unsupported message type: {other}")),
    }
}

fn round_trip<'a, M>(payload: &'a [u8]) -> ConformanceResponse<'static>
where
    M: MessageRead<'a> + MessageWrite,
{
    // Deserialize
    let mut reader = BytesReader::from_bytes(payload);
    let msg = match M::from_reader(&mut reader, payload) {
        Ok(m) => m,
        Err(e) => {
            return make_error(format!("parse error: {e}"));
        }
    };

    // Re-serialize
    let size = msg.get_size();
    let mut out = vec![0u8; size];
    let mut writer = Writer::new(&mut out[..]);
    match msg.write_message(&mut writer) {
        Ok(()) => ConformanceResponse {
            result: OneOfresult::protobuf_payload(Cow::Owned(out)),
        },
        Err(e) => ConformanceResponse {
            result: OneOfresult::serialize_error(Cow::Owned(format!("serialize error: {e}"))),
        },
    }
}

fn make_skipped(reason: &str) -> ConformanceResponse<'static> {
    ConformanceResponse {
        result: OneOfresult::skipped(Cow::Owned(reason.to_string())),
    }
}

fn make_error(msg: String) -> ConformanceResponse<'static> {
    ConformanceResponse {
        result: OneOfresult::parse_error(Cow::Owned(msg)),
    }
}
