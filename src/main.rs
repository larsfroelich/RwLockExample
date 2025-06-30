use std::sync::RwLock;
use std::thread;
use std::time::Duration;

fn main() {
    let rwlock = RwLock::new(Some(1));
    
    // count 1 to 3
    for _ in 0..3 {
        if let Some(nr) = {rwlock.read().unwrap().clone()} {
            println!("Read value: {}", nr);
            println!("Writing value n+1...");
            // this write-lock doesn't block because the read lock is released
            // after being temporarily held inside its own statement scope {}
            *rwlock.write().unwrap() = Some(nr + 1);
        } else {
            println!("Failed to read value");
        }
        thread::sleep(Duration::from_millis(500));
    }
    
    // count 4 to 6
    for _ in 0..3 {
        if let Some(nr) = rwlock.read().unwrap().clone() {
            println!("Read value: {}", nr);
            println!("Writing value n+1...");
            // THIS WRITE LOCK WILL BLOCK BECAUSE THE READ LOCK IS STILL HELD
            *rwlock.write().unwrap() = Some(nr + 1);
        } else {
            println!("Failed to read value");
        }
        thread::sleep(Duration::from_millis(500));
    }
}

