pub struct RaftConfig {
    pub election_timeout: u64,
    pub heartbeat_interval: u64,
}

impl Default for RaftConfig {
    fn default() -> Self {
        Self {election_timeout: 300, heartbeat_interval: 150 }
    }
}