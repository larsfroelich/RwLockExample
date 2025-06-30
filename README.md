# RwLockExample

This project demonstrates correct and incorrect usage of `std::sync::RwLock` in Rust. It stores a number inside a lock and shows how keeping a read lock while trying to acquire a write lock causes blocking.

See the raw code: [here](https://gist.github.com/rust-play/1bfa4a3bf67235cee422b9576245aa09)

Try in Rust Playground: [here](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&code=use+std%3A%3Async%3A%3ARwLock%3B%0D%0Ause+std%3A%3Athread%3B%0D%0Ause+std%3A%3Atime%3A%3ADuration%3B%0D%0A%0D%0Afn+main%28%29+%7B%0D%0A++++let+rwlock+%3D+RwLock%3A%3Anew%28Some%281%29%29%3B%0D%0A++++%0D%0A++++%2F%2F+count+1+to+3%0D%0A++++for+_+in+0..3+%7B%0D%0A++++++++if+let+Some%28nr%29+%3D+%7Brwlock.read%28%29.unwrap%28%29.clone%28%29%7D+%7B%0D%0A++++++++++++println%21%28%22Read+value%3A+%7B%7D%22%2C+nr%29%3B%0D%0A++++++++++++println%21%28%22Writing+value+n%2B1...%22%29%3B%0D%0A++++++++++++%2F%2F+this+write-lock+doesn%27t+block+because+the+read+lock+is+released%0D%0A++++++++++++%2F%2F+after+being+temporarily+held+inside+its+own+statement+scope+%7B%7D%0D%0A++++++++++++*rwlock.write%28%29.unwrap%28%29+%3D+Some%28nr+%2B+1%29%3B%0D%0A++++++++%7D+else+%7B%0D%0A++++++++++++println%21%28%22Failed+to+read+value%22%29%3B%0D%0A++++++++%7D%0D%0A++++++++thread%3A%3Asleep%28Duration%3A%3Afrom_millis%28500%29%29%3B%0D%0A++++%7D%0D%0A++++%0D%0A++++%2F%2F+count+4+to+6%0D%0A++++for+_+in+0..3+%7B%0D%0A++++++++if+let+Some%28nr%29+%3D+rwlock.read%28%29.unwrap%28%29.clone%28%29+%7B%0D%0A++++++++++++println%21%28%22Read+value%3A+%7B%7D%22%2C+nr%29%3B%0D%0A++++++++++++println%21%28%22Writing+value+n%2B1...%22%29%3B%0D%0A++++++++++++%2F%2F+THIS+WRITE+LOCK+WILL+BLOCK+BECAUSE+THE+READ+LOCK+IS+STILL+HELD%0D%0A++++++++++++*rwlock.write%28%29.unwrap%28%29+%3D+Some%28nr+%2B+1%29%3B%0D%0A++++++++%7D+else+%7B%0D%0A++++++++++++println%21%28%22Failed+to+read+value%22%29%3B%0D%0A++++++++%7D%0D%0A++++++++thread%3A%3Asleep%28Duration%3A%3Afrom_millis%28500%29%29%3B%0D%0A++++%7D%0D%0A%7D%0D%0A)

## Usage

Run the example with:

```bash
cargo run
```

The first loop succeeds because the read lock is scoped and released before the write lock is taken. The second loop blocks when a write lock is requested while the read lock is still held.
