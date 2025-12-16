//! Protobuf serialization and deserialization benchmark using prost.
//!
//! This benchmark tests Protocol Buffers encoding and decoding performance
//! on a realistic data structure representing user data.

use prost::Message;
use sightglass_api as bench;

// Include the generated protobuf code from OUT_DIR
#[allow(non_snake_case)]
mod proto {
    include!(concat!(env!("OUT_DIR"), "/benchmark.rs"));
}

use proto::*;

fn main() {
    // Read the protobuf binary data
    let data = std::fs::read("default.input").expect("unable to read default.input");

    bench::start();

    // Deserialize: Decode protobuf bytes into structured data
    let user_list = UserList::decode(&data[..]).expect("failed to decode protobuf");

    // Serialize: Encode back to protobuf bytes
    let mut buf = Vec::new();
    user_list
        .encode(&mut buf)
        .expect("failed to encode protobuf");

    bench::end();

    eprintln!("[rust-protobuf] processed {} users", user_list.users.len());
    eprintln!("[rust-protobuf] serialized size: {} bytes", buf.len());
}
