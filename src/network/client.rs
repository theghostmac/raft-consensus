use crate::raft::rpc::{RequestVote, AppendEntries};
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde_json::json;
use crate::raft::node::RaftNode;

impl RaftNode {
    pub async fn send_request_vote(&self, peer: &str, request_vote: RequestVote) -> bool {
        let mut stream = TcpStream::connect(peer).await.unwrap();
        let request = json!({
            "type": "RequestVote",
            "term": request_vote.term,
            "candidate_id": request_vote.candidate_id,
            "last_log_index": request_vote.last_log_index,
            "last_log_term": request_vote.last_log_term,
        });
        let request = serde_json::to_vec(&request).unwrap();
        stream.write_all(&request).await.unwrap();

        let mut buffer = [0; 1024];
        stream.read(&mut buffer).await.unwrap();
        let response: serde_json::Value = serde_json::from_slice(&buffer).unwrap();
        response["voteGranted"].as_bool().unwrap_or(false)
    }

    pub async fn send_append_entries(&self, peer: &str, append_entries: AppendEntries) -> bool {
        let mut stream = TcpStream::connect(peer).await.unwrap();
        let request = json!({
            "type": "AppendEntries",
            "term": append_entries.term,
            "leader_id": append_entries.leader_id,
            "prev_log_index": append_entries.prev_log_index,
            "prev_log_term": append_entries.prev_log_term,
            "entries": append_entries.entries,
            "leader_commit": append_entries.leader_commit,
        });
        let request = serde_json::to_vec(&request).unwrap();
        stream.write_all(&request).await.unwrap();

        let mut buffer = [0; 1024];
        stream.read(&mut buffer).await.unwrap();
        let response: serde_json::Value = serde_json::from_slice(&buffer).unwrap();
        response["success"].as_bool().unwrap_or(false)
    }
}
