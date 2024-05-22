# Raft Consensus Algorithm

Servers are susceptible to failures, such as disc failure. This is critical
if you're running one server instance. It would take time to address/resolve
the failure. And down-time is bad.


To mitigate failure, spin up multiple servers. Even if one fails and goes offline,
others will work together to keep the service online - this is a distributed system.


The multiple servers are nodes and are copies of each other. While they solve the
problem of having a single point of failure, they bring up a new problem: they
need to stay in sync and have data consistency - so that they have the same data at the same
time.

All replicas/nodes must have the same data at the same time, or more importantly,
all nodes must agree that a particular record of data already exists in at least one
node's storage (just in case it doesn't sync on time before the data is being accessed).

To achieve this agreement/consensus on the availability of the data, a consensus algorithm
is used.

A consensus algorithm coordinates all of the nodes within a distributed system to come
to an agreement or achieve consensus on a data. Example of consensus algorithms include:

- Proof of work: involves nodes solving difficult mathematical puzzles to validate transactions and record them on the blockchain. This is also know as mining.
- Paxos Algorithm: used by Apache Zookeeper, difficult to understand and implement.
- Raft Algorithm: an understandable algorithm compared to Paxos.

## Raft
Raft tackles the problem of consensus through single-leader election and log replication.

Imagine a distributed key-value server that runs on a cluster of three nodes. Each node/replica
holds a state machine, log, and raft protocol. A state machine is a program that is replicated.
For this example, the state machine is a server that has endpoints for the key-value store.

As long as they begin the same state, and perform the same operations in the same order, they
should all end up with the same state: state machine replication.

When a new command enters one of the replicas, it appends and saves the command as a new entry in its log.
These commands get fed to the replica's state machine as input. Every replica's log must always contain the
exact sequence of commands for the replicas to remain synchronized.

The single-leader election is necessary to have a single node/replica that is responsible for sending commands
to the replicas.

### States
Each node can assume all states, but is only capable of taking one state at a time: follower, candidate, leader. 
All replicas start out as a follower state. Follower
