use crate::raft::node::RaftNode;
use crate::raft::rpc::AppendEntries;

impl RaftNode {
    pub async fn handle_append_entries(&mut self, request: AppendEntries) -> bool {
        if request.term < self.current_term {
            return false;
        }

        self.current_term = request.term;
        if request.prev_log_index > self.log.last_index() {
            return false;
        }

        if let Some(prev_entry) = self.log.entries.get(request.prev_log_index as usize) {
            if prev_entry.term != request.prev_log_term {
                return false;
            }
        }

        for entry in request.entries {
            self.log.append(entry);
        }

        if request.leader_commit > self.commit_index {
            self.commit_index = request.leader_commit;
        }

        true
    }
}
