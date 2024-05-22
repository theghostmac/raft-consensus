use crate::raft::node::RaftNode;
use crate::raft::rpc::{RequestVote, AppendEntries};
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde_json::json;

impl RaftNode {
    pub async fn start_server(&mut self) {
        let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
        loop {
            let (mut socket, _) = listener.accept().await.unwrap();
            let mut buffer = [0; 1024];
            socket.read(&mut buffer).await.unwrap();
            let request: serde_json::Value = serde_json::from_slice(&buffer).unwrap();
            let response = match request["type"].as_str() {
                Some("RequestVote") => {
                    let request_vote: RequestVote = serde_json::from_value(request).unwrap();
                    let granted = self.handle_request_vote(request_vote).await;
                    json!({ "voteGranted": granted })
                }
                Some("AppendEntries") => {
                    let append_entries: AppendEntries = serde_json::from_value(request).unwrap();
                    let success = self.handle_append_entries(append_entries).await;
                    json!({ "success": success })
                }
                _ => json!({ "error": "Unknown request type" }),
            };
            let response = serde_json::to_vec(&response).unwrap();
            socket.write_all(&response).await.unwrap();
        }
    }
}
