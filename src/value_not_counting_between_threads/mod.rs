use std::{sync::{atomic::{AtomicUsize, Ordering}, Arc, Mutex, RwLock}, thread};


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

    println!("Final counter value: {}", *counter.lock().unwrap());
    *counter.clone().lock().unwrap()
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

    println!("Final counter value: {}", *counter.try_read().unwrap());
    let final_counter_value = *counter.read().unwrap();
    final_counter_value
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

    println!("Final counter value: {}", counter.load(Ordering::Relaxed));
    counter.load(Ordering::Relaxed)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(arc_mutex_solution(), 1000);
        assert_eq!(arc_rwlock_solution(), 1000);
        assert_eq!(arc_atomic_usize_solution(), 1000);
    }
}