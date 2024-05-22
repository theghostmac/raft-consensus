use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};
use log::info;
use crate::raft::config::RaftConfig;
use crate::raft::log::{Log, LogEntry};
use crate::raft::rpc::{AppendEntries, RequestVote};
use crate::raft::state::State;

pub struct RaftNode {
    config: RaftConfig,
    state: State,
    pub(crate) log: Log,
    pub(crate) current_term: u64,
    pub(crate) voted_for: Option<u64>,
    pub(crate) commit_index: u64,
    last_applied: u64,
    election_timeout: u64,
    heartbeat_interval: u64,
    peers: Vec<u64>,
    id: u64,
}

impl RaftNode {
    pub fn new(config: RaftConfig, id: u64, peers: Vec<u64>) -> Self {
        let election_timeout = config.election_timeout;
        let heartbeat_interval = config.heartbeat_interval;

        Self {
            config,
            state: State::Follower,
            log: Log::new(),
            current_term: 0,
            voted_for: None,
            commit_index: 0,
            last_applied: 0,
            election_timeout,
            heartbeat_interval,
            peers,
            id,
        }
    }

    pub async fn start(&mut self) {
        info!("Node {} started as a follower", self.id);
        self.run().await;
    }

    async fn run(&mut self) {
        loop {
            match self.state {
                State::Follower => {
                    info!("Node {} is in Follower state", self.id);
                    self.run_follower().await;
                },
                State::Candidate => {
                    info!("Node {} is in Candidate state", self.id);
                    self.run_candidate().await;
                },
                State::Leader => {
                    info!("Node {} is in Leader state", self.id);
                    self.run_leader().await;
                },
            }
        }
    }

    async fn run_follower(&mut self) {
        let timeout = Duration::from_millis(self.election_timeout);
        info!("Node {} waiting for {} ms as follower", self.id, self.election_timeout);
        sleep(timeout).await;

        // If this follower didn't receive any heartbeat, it becomes a candidate, and starts an election.
        if self.state == State::Follower {
            info!("Node {} did not receive heartbeat, becoming candidate", self.id);
            self.state = State::Candidate;
        }
    }

    async fn run_candidate(&mut self) {
        self.current_term += 1;
        self.voted_for = Some(self.id);
        info!("Node {} started election for term {}", self.id, self.current_term);

        // Send RequestVote RPCs to all peers.
        let request_vote = RequestVote {
            term: self.current_term,
            candidate_id: self.id,
            last_log_index: self.log.entries.len() as u64,
            last_log_term: self.log.entries.last().map_or(0, |entry| entry.term),
        };

        // Simulate sending RPCs to peers
        // TODO: replace with actual network calls.
        let votes = Arc::new(Mutex::new(1)); // Vote for self.
        for peer in &self.peers {
            let votes = Arc::clone(&votes);
            let request_vote = request_vote.clone();
            tokio::spawn(async move {
                // Simulate network request.
                // TODO: replace with actual network calls.
                sleep(Duration::from_millis(100)).await;

                // Simulate peer response
                // TODO: replace with actual RPC calls.
                let vote_granted = true; // Simulate that the vote was granted.
                if vote_granted {
                    let mut votes = votes.lock().unwrap();
                    *votes += 1;
                }
            });
        }

        // Wait for a majority of votes / election timeout.
        let timeout = Duration::from_millis(self.election_timeout);
        sleep(timeout).await;

        // Check if received a majority of votes.
        let votes = votes.lock().unwrap();
        if *votes > self.peers.len() / 2 {
            info!("Node {} received majority votes, becoming leader", self.id);
            self.state = State::Leader;
        } else {
            info!("Node {} did not receive majority votes, remaining follower", self.id);
            self.state = State::Follower;
        }
    }

    async fn run_leader(&mut self) {
        // Send heartbeats to all followers.
        let append_entries = AppendEntries {
            term: self.current_term,
            leader_id: self.id,
            prev_log_index: self.log.entries.len() as u64,
            prev_log_term: self.log.entries.last().map_or(0, |e| e.term),
            entries: vec![],
            leader_commit: self.commit_index,
        };

        for peer in &self.peers {
            let append_entries = append_entries.clone();
            tokio::spawn(async move {
                // Simulate network request.
                // TODO: replace with actual network calls.
                sleep(Duration::from_millis(50)).await;

                // Simulate peer response.
                // TODO: replace with actual RPC calls.
                let success = true; // Simulate that the heartbeat was successful.
                if success {
                    // TODO: handle heartbeat response.
                }
            });
        }

        // Wait for a heartbeat timeout.
        let timeout = Duration::from_millis(self.heartbeat_interval);
        info!("Node {} sending heartbeats every {} ms as leader", self.id, self.heartbeat_interval);
        sleep(timeout).await;
    }

    // TODO: add any other node methods needed to implement the Raft protocol.
}
