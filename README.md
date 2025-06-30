# RwLockExample

This project demonstrates correct and incorrect usage of `std::sync::RwLock` in Rust. It stores a number inside a lock and shows how keeping a read lock while trying to acquire a write lock causes blocking.

## Usage

Run the example with:

```bash
cargo run
```

The first loop succeeds because the read lock is scoped and released before the write lock is taken. The second loop blocks when a write lock is requested while the read lock is still held.
