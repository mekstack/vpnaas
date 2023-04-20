pub mod proto {
    tonic::include_proto!("vpnaas");
}

pub use proto::confus_server::ConfusServer;
pub use proto::keys_client::KeysClient;
