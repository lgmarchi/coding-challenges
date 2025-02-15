use std::{sync::{atomic::{AtomicUsize, Ordering}, mpsc::{self, Receiver, Sender}, Arc, Mutex, RwLock}, thread};

pub fn arc_mutex_solution() -> usize {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                let mut num = counter.lock().unwrap();
                *num += 1;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // println!("Final counter value: {}", *counter.lock().unwrap());
    *counter.clone().lock().unwrap()
}

// This does not work for complex data like Vec or String
pub fn arc_atomic_usize_solution() -> usize {
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                counter.fetch_add(1, Ordering::Relaxed);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // println!("Final counter value: {}", counter.load(Ordering::Relaxed));
    counter.load(Ordering::Relaxed)
}


pub fn arc_rwlock_solution() -> usize {
    let counter = Arc::new(RwLock::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                let mut num = counter.write().unwrap();
                *num += 1;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // println!("Final counter value: {}", *counter.try_read().unwrap());
    let final_counter_value = *counter.read().unwrap();
    final_counter_value
}

pub fn standard_channels_solution() -> usize {
    let mut counter = 0;
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
    let mut handles = vec![];

    for _ in 0..10 {
        let tx_clone = tx.clone();
        let mut thread_counter = 0;
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                thread_counter += 1;
              
            }
            tx_clone.send(thread_counter).unwrap();
        });
        handles.push(handle);
    }
    // Important to drop tx to let rx know when to stop
    drop(tx);

    // For multiple threads
    for received in rx {
        counter += received;
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let final_counter_value = counter;
    // println!("Final counter value: {}", counter);
    final_counter_value as usize
}

pub fn crossbeam_channels_solution() -> usize {
    let mut counter = 0;
    let (tx, rx) = crossbeam::channel::unbounded();
    let mut handles = vec![];

    for _ in 0..10 {
        let tx_clone = tx.clone();
        // Using mutable counter with sender will cause data race condition
        // blocking the app
        let mut thread_counter = 0;
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                thread_counter += 1;
              
            }
            tx_clone.send(thread_counter).unwrap();
        });
        handles.push(handle);
    }
    // Important to drop tx to let rx know when to stop
    drop(tx);

    // For multiple threads
    for received in rx {
        counter += received;
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let final_counter_value = counter;
    // println!("Final counter value: {}", counter);
    final_counter_value as usize
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(arc_mutex_solution(), 1000);
        assert_eq!(arc_atomic_usize_solution(), 1000);
        assert_eq!(arc_rwlock_solution(), 1000);
        assert_eq!(standard_channels_solution(), 1000);
        assert_eq!(crossbeam_channels_solution(), 1000);
    }
}