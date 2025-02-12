# **Race Condition in Rust: Shared Counter Issue**

## **Problem Description**
This Rust program attempts to use multiple threads to increment a shared counter. However, there is a critical issue in the implementation that prevents the counter from updating correctly.

## **Code**
```rust
fn main() {
    let mut counter = 0;
    let mut handles = vec![];

    for _ in 0..10 {
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                counter = counter + 1;
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final counter value: {}", counter);
}
```

## **Issue Explanation**
### **1. Race Condition**
- The `counter` variable is declared as a mutable integer, but it is not safely shared across multiple threads.
- Each thread receives its own copy of `counter` due to Rust's ownership and move semantics.
- As a result, the changes made inside the spawned threads are not reflected in the main thread.

### **2. Data Races and Thread Safety**
- Since each thread operates on its own instance of `counter`, the expected increment operations are not performed on the shared variable.
- Rust prevents direct sharing of mutable data between threads unless explicitly synchronized.

## **Expected Output vs. Actual Output**
- The expected output should be `Final counter value: 1000` (10 threads * 100 increments each).
- However, the actual output remains `0` because the main thread's `counter` is never modified by the spawned threads.

## **Conclusion**
This problem highlights the importance of **thread safety** and **data synchronization** when working with concurrency in Rust.
