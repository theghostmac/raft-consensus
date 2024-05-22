use crate::raft::node::RaftNode;
use crate::raft::config::RaftConfig;

mod raft;
mod network;
mod utils;

#[tokio::main]
async fn main() {
    env_logger::init();
    let config = RaftConfig::default();
    let id = 1; // Example node ID
    let peers = vec![2, 3]; // Example peer IDs

    let mut node = RaftNode::new(config, id, peers);
    node.start().await;
}
