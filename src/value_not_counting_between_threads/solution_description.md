# **Thread-Safe Counter Solutions in Rust**

## **Introduction**
When working with multi-threading in Rust, ensuring safe access to shared data is crucial. This document explains three different solutions for a shared counter using `Arc<Mutex<T>>`, `Arc<RwLock<T>>`, and `Arc<AtomicUsize>`. Each solution has its use case, advantages, and trade-offs.

---

## **1️⃣ Arc + Mutex Solution**

### **How it Works**
- Uses **`Arc<Mutex<T>>`** to allow multiple threads to safely modify a shared counter.
- Each thread locks the `Mutex`, increments the counter, and then releases the lock.
- Ensures exclusive access to prevent race conditions.

### **Code**
```rust
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
    *counter.lock().unwrap()
}
```

### **Best Use Cases**
✅ When multiple threads need to modify shared data.
✅ When data integrity is more important than speed.
✅ Works well for complex data structures like `Vec<T>` or `String`.

### **Drawbacks**
❌ Locks introduce performance overhead.
❌ If one thread panics while holding the lock, the mutex remains poisoned.

---

## **2️⃣ Arc + RwLock Solution**

### **How it Works**
- Uses **`Arc<RwLock<T>>`** to allow multiple readers but ensures exclusive write access.
- Threads obtain a write lock when updating the counter.
- Multiple readers can access the counter concurrently.

### **Code**
```rust
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

    println!("Final counter value: {}", *counter.read().unwrap());
    *counter.read().unwrap()
}
```

### **Best Use Cases**
✅ When most operations involve reading data rather than writing.
✅ Works well for shared configurations or caches.
✅ Allows multiple readers, reducing contention compared to `Mutex<T>`.

### **Drawbacks**
❌ Writing still requires exclusive access (blocking readers).
❌ Slightly more overhead than `Mutex<T>` when contention is high.

---

## **3️⃣ Arc + AtomicUsize Solution**

### **How it Works**
- Uses **`Arc<AtomicUsize>`** to perform atomic operations without requiring locks.
- Each thread calls `fetch_add(1, Ordering::Relaxed)` to increment the counter safely.
- Avoids the overhead of locking mechanisms.

### **Code**
```rust
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
```

### **Best Use Cases**
✅ When only numerical counters are needed.
✅ Best for high-performance applications where locks introduce too much overhead.
✅ Perfect for simple counters, metrics, and statistical data.

### **Drawbacks**
❌ Cannot be used with complex data like `Vec<T>` or `String`.
❌ Ordering guarantees must be carefully considered (`Relaxed`, `Acquire`, `Release`).

---

## **Conclusion: When to Use Each Approach?**

| Approach               | Best For                          | Performance |
|------------------------|---------------------------------|-------------|
| `Arc<Mutex<T>>`       | Shared mutable data (Vec, String) | Medium      |
| `Arc<RwLock<T>>`      | Read-heavy workloads             | High (Reads) |
| `Arc<AtomicUsize>`    | Simple numerical counters        | Very High   |

### **Final Recommendation**
- Use **`Mutex<T>`** for **complex shared data** (e.g., `Vec<T>`, `String`).
- Use **`RwLock<T>`** when **reading is more frequent** than writing.
- Use **`AtomicUsize`** when you only need a **simple counter** without complex data.

Choosing the right synchronization mechanism depends on the **trade-offs** between performance, complexity, and safety. 🚀

