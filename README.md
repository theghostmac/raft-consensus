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

## Raft's Working Principle
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
All replicas start out as a follower state. Follower nodes/replicas can only accept commands.

If no leader is present or the is unresponsive, the followers must elect a new leader. A leader is reponsible for
receiving requests from a client and sending commands to followers.

Only the leader can receive requests from a client. In case the client tries to send a request to a follower, a 
load balancer is placed in front of the cluster to redirect the request to the leader.

### Elections
Each follower sets an election time out, which is a specific time interval withing which the follower must hear
back from a leader. Raft randomizes the election time-out for each follower, but it typically falls within
the range of 150ms to 300ms.

The moment a follower reaches its election time-out and it doesn't hear back from the leader, the follower
becomes a candidate, initiates an election for a new leader, and votes for itself. To request votes from other
followers, the candidate sends a request-vote (`RequestVote`) message to them, and waits for their vote response.

Request vote is one of two types of remote procedure calls (RPCs) used by Raft for in-cluster communication.

The message includes information about the total number of entries in the candidate's log, and the term
of the latest entry. A term is a counter value that represents an arbitrary time period during the lifetime
of a Raft cluster. Each replica starts with `term=0`, and each of them maintains its own term.
The term increments anytime an election begins. 

Elections begin for different purposes:
- a leader goes offline
- network experiences high latency, making a follower reach its election timeout despite a leader still being alive.

Followers will not vote for the candidate if there are any inconsistencies in the candidate's log. If the
candidate receives the majority of votes from the other followers, then the candidate becomes the new leader.
If the candidate is not elected, it becomes a follower again.

Once a leader has been elected, it sends `AppendEntries` messages to followers in the cluster.

Append Entries is the second type of remote procedure calls used by Raft for in-cluster communication.

It serves as both a heart-beat mechanism, and tells followers to replicate new log entries.

A heart-beat time-out determines how often these messages are sent to followers, so that they know the leader
is still alive.

## Client -> Leader operation
Client sends a reques to the leader to set a data in the store.
The leader appends the set operation as a new entry in it's log.

Appending the new entry in the log does not actually perform the operation. The entry is first committed
for the operation to be performed. This would warrant that the majority of the followers have that entry
appended to their logs. The leader sends `AppendEntries` messages to all followers. Each follower performs
a consistency check to verify that its log is identical to the leader's. After passing the check, the
follower accepts/appends the set operation as a new entry in it's log.

Once the majority of followers have written the new entry to their logs, the leader commits the entry and
applies it to the state machine. so we now have a replica in our cluster that has updated their data store with
the new data. Then the leader sends `AppendEntries` messages to the followers, but this time, to notify followers
that the entry has been committed, and that they too should commit the entry. This is consensus. This is known
as log replication.


Having many clients place requests to a single leader can become a bottleneck, especially when the leader requires
acknowledgement from its followers, however, its still in use. HashiCorp Tech used it in many of their products, like
Consul, Nomad, Vault. MongoDB and CockroachDB used it.

CockroachDB has a blog post on scaling Raft.

Read NuRaft (C++) by eBay, or Raft by HashiCorp.
