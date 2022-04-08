pub mod broker {
    tonic::include_proto!("broker"); // The string specified here must match the proto package name
}

pub mod topic;
pub mod brokerimpl;