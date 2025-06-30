# RwLock Example

A simple Rust project demonstrating the usage of `RwLock` (Read-Write Lock) from the standard library, highlighting both correct and problematic usage patterns.

## About

This project shows how to use Rust's `std::sync::RwLock` to safely share mutable data between different parts of your code. It demonstrates:

1. **Correct usage**: Properly releasing a read lock before acquiring a write lock
2. **Problematic usage**: Attempting to acquire a write lock while still holding a read lock, which leads to blocking

## What is RwLock?

`RwLock` (Read-Write Lock) is a synchronization primitive that allows multiple readers or a single writer to access shared data at any given time:

- Multiple threads can read the data simultaneously (shared access)
- Only one thread can write to the data, and no other thread can access it during writing (exclusive access)
- Writers have priority over readers to prevent writer starvation

## Example Explained

The example in this project demonstrates two scenarios:

### Scenario 1: Correct Usage (Lines 9-20)
```rust
// The read lock is released after being used within its own scope
if let Some(nr) = {rwlock.read().unwrap().clone()} {
    println!("Read value: {}", nr);
    println!("Writing value n+1...");
    // This write-lock doesn't block because the read lock is released
    // after being temporarily held inside its own statement scope {}
    *rwlock.write().unwrap() = Some(nr + 1);
}
```

In this scenario, the read lock is acquired within a scope `{...}` and is automatically released when that scope ends. This allows the write lock to be acquired without blocking.

### Scenario 2: Problematic Usage (Lines 23-32)
```rust
// The read lock is held throughout the if-let block
if let Some(nr) = rwlock.read().unwrap().clone() {
    println!("Read value: {}", nr);
    println!("Writing value n+1...");
    // THIS WRITE LOCK WILL BLOCK BECAUSE THE READ LOCK IS STILL HELD
    *rwlock.write().unwrap() = Some(nr + 1);
}
```

In this scenario, the read lock is held throughout the entire if-let block. When the code tries to acquire a write lock while still holding the read lock, it will block indefinitely, potentially causing a deadlock.

## Running the Example

To run this example:

```bash
cargo run
```

You'll observe that the first loop (correct usage) completes successfully, while the second loop (problematic usage) will likely hang due to the blocking behavior.

## Project Details

- **Name**: RwLockExample
- **Version**: 0.1.0
- **Rust Edition**: 2024

## Key Takeaways

1. Always ensure read locks are released before attempting to acquire write locks
2. Use scoping `{...}` to limit the lifetime of locks
3. Be mindful of lock acquisition order to prevent deadlocks
4. Consider using `parking_lot::RwLock` for more advanced use cases, as it offers better performance and additional features