# volaKV

A high-performance, asynchronous, in-memory key-value store built with Rust.

`volaKV` is designed for low-latency operations, predictable concurrency, and high throughput using modern asynchronous I/O primitives.

---

## ⚡ Features (Target Architecture)

- **Asynchronous Core**: Powered by Tokio runtime with non-blocking I/O.
- **Custom Wire Protocol**: Lightweight, binary/text framing protocol designed for minimal parsing overhead.
- **Concurrent Storage Engine**: Thread-safe in-memory key-value engine with fine-grained locking or lock-free partition strategies.
- **TTL & Expiration**: Background eviction and lazy deletion of expired keys.
- **Persistence (Planned)**: Append-Only File (AOF) and snapshot persistence options.
- **Metrics & Observability**: Real-time stats on latency, memory usage, and throughput.

---

## 🗺️ Roadmap & Milestones

### Phase 1: Foundation & In-Memory Engine
- [x] Initialize Rust workspace and configure tooling (formatting, linting).
- [x] Implement core data structure for key-value storage (`SET`, `GET`, `DEL`, `EXISTS`).
- [x] Integrate thread-safe synchronization.
- [x] Basic unit and integration test suite for engine operations.

### Phase 2: Asynchronous Networking & Framing
- [ ] Setup TCP server utilizing Tokio.
- [ ] Define wire protocol specifications and frame serialization/deserialization.
- [ ] Implement asynchronous connection handler and command dispatcher.
- [ ] Concurrency and connection pooling benchmarks.

### Phase 3: Advanced Storage Features
- [ ] Key expiration (TTL support, passive expiration on access, active background cleaner).
- [ ] Extended data structures (e.g., Lists, Sets, or Hashes).
- [ ] Memory limit policy & eviction strategies (e.g., LRU/LFU).

### Phase 4: Persistence & Durability
- [ ] Append-Only File (AOF) write-ahead logging.
- [ ] Background compaction/rewriting of AOF logs.
- [ ] Snapshotting (RDB-style point-in-time state export).

### Phase 5: Production Readiness & Tooling
- [ ] CLI client (`volakv-cli`) for interacting with the server.
- [ ] Comprehensive stress tests and automated benchmark suites (`criterion`).
- [ ] CI/CD workflows (formatting, clippy, multi-platform builds).
- [ ] Docker containerization and deployment guides.

---

## 🚀 Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/) (latest stable toolchain recommended)
- `cargo`

### Building & Running

```bash
# Clone the repository
git clone https://github.com/KsanneK/volaKV.git
cd volaKV

# Build the project in release mode
cargo build --release

# Run tests
cargo test
```

---

## 📄 License

This project is licensed under the terms specified in the [LICENSE](file:///home/ksanek/Documents/Projects/volaKV/LICENSE) file.
