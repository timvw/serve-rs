pub mod broker {
    tonic::include_proto!("broker"); // The string specified here must match the proto package name
}

#[allow(dead_code)]
pub mod bitvavo;

pub mod mybroker;