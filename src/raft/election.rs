use crate::raft::node::RaftNode;
use crate::raft::rpc::RequestVote;

impl RaftNode {
    pub async fn handle_request_vote(&mut self, request: RequestVote) -> bool {
       if request.term < self.current_term {
           return false;
       }

        if self.voted_for.is_none() || self.voted_for == Some(request.candidate_id) {
            self.voted_for = Some(request.candidate_id);
            self.current_term = request.term;
            return true;
        }

        false
    }
}