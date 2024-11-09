# CRDT WebSocket Editor
A WebSocket-based server implemented in Python with a Rust-backed CRDT (Conflict-free Replicated Data Type) functionality, providing real-time data synchronization and persistence. The server enables clients to connect, add, update, and remove key-value pairs, with all operations broadcast to connected clients.

[Check out the live demo here!](https://crdt.martishin.com/)

<img src="https://i.giphy.com/media/v1.Y2lkPTc5MGI3NjExdTQ2bjIybDJ1cWN1dWlud3dmOHFnOGR6MmhrNTdjYmR1eHpnaWxjaSZlcD12MV9pbnRlcm5hbF9naWZfYnlfaWQmY3Q9Zw/CxGDTYaEOq1vVKUEMe/giphy.gif" width="700"/>

## Features
* Blazingly Fast & Reliable CRDT - Last-Write-Wins (LWW) Element Dictionary for conflict resolution  
* Real-Time Synchronization - Any changes to data are instantly reflected on all connected clients

## Running Locally

### Prerequisites
- Python & Conda - for running the serer
- Node.js & npm - for running the React client
- Rust - for building the CRDT library

### Build the CRDT Library
```bash
make build-lib
```

### Run the Server
```bash
make run-server
```

### Run the Client
```bash
make run-client
```

### Run Tests
```bash
make test
```

## Algorithm
The LWW-Element-Dictionary CRDT is a data structure that allows multiple replicas to manage data independently and concurrently, resolving any inconsistencies that may arise. Here’s a deeper look at its functionality:

### Conflict Resolution
Each entry in the dictionary has:

* A timestamp marking when it was last modified  
* The value itself, paired with its timestamp  

When a key’s value is updated, if the incoming timestamp is later than the existing one, the new value overwrites the old one. If a remove operation is applied with a more recent timestamp, the key is marked as removed, and any additions with earlier timestamps are ignored.

### Key Operations
* Add: Adds or updates a key-value pair if the incoming timestamp is more recent than the current one.
* Remove: Marks a key as removed, only if the remove timestamp is later than any existing timestamps.
* Lookup: Retrieves the value for a key if it exists and hasn’t been marked as removed.
* Merge: Combines another dictionary with this one, resolving any conflicts based on timestamps, which allows distributed replicas to synchronize data effectively.

The CRDT approach ensures that changes made by any client will eventually propagate to all others, and the Last-Write-Wins strategy guarantees deterministic conflict resolution.

## Technologies Used
* TypeScript (Client) - A front-end built with Vite and React, providing a responsive interface for real-time data editing
* Python (Server) - The FastAPI WebSocket server handles connections, synchronization, and data persistence
* Rust (CRDT Library) - A high-performance CRDT library providing a Last-Write-Wins Element Dictionary for conflict resolution

This project is a powerful demonstration of cross-language interoperability, with each layer of the stack contributing to a robust, real-time distributed system.
