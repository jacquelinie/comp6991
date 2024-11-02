1) I saw someone's code fail to compile because they
were trying to send non-thread-safe data across threads.
How does the Rust language allow for static (i.e. at compile time)
guarantees that specific data can be sent/shared acrosss threads?

Rust uses the Send and Sync traits to enforce thread safety. Types that implement Send can be transferred across threads, and types that implement Sync can be shared between threads. These traits are automatically implemented by the Rust compiler for types that are inherently thread-safe, allowing Rust to perform static checks at compile time to ensure that only types with these traits can be used in multithreaded contexts. This way, Rust prevents data races and ensures safe concurrency without needing a runtime check.

2) Do you have to then implement the Send and Sync traits for
every piece of data (i.e. a struct) you want to share and send across threads?

No, you don’t have to implement Send and Sync manually for most types. Rust’s compiler will automatically mark types as Send or Sync if they’re composed only of types that are also Send or Sync, following Rust’s rules for thread safety. Manual implementation is rare and is typically only necessary for types that require custom concurrency handling, such as types that manage non-thread-safe raw pointers.

3) What types in the course have I seen that aren't Send? Give one example,
and explain why that type isn't Send.

An example of a type that isn’t Send is Rc<T>, Rust’s reference-counting pointer. Rc<T> is not thread-safe because it isn’t atomic and doesn’t have internal locking mechanisms, which means it could lead to data races if shared across threads. Instead, Rust provides Arc<T> (atomic reference counting), which is thread-safe and thus implements Send.

4) What is the relationship between Send and Sync? Does this relate
to Rust's Ownership system somehow?

Send and Sync are closely related but serve different purposes:
Send means that a type can be safely moved to another thread.
Sync means that a type can be safely accessed from multiple threads simultaneously, provided the access is read-only or protected by synchronization primitives like mutexes.

Rust’s ownership model ensures that Send types are moved rather than copied when transferred to other threads, enforcing unique ownership. Similarly, Sync ensures that data shared across threads is accessed safely, following Rust’s ownership and borrowing rules, which restrict simultaneous mutable access unless managed by safe concurrency patterns.

5) Are there any types that could be Send but NOT Sync? Is that even possible?

Yes, it is possible for a type to be Send but not Sync. A common example is Cell<T> or RefCell<T>. These types enable interior mutability, allowing modification of data even when there are immutable references to it. They can be Send if only one thread has ownership of them, but they cannot be Sync because allowing concurrent access could cause data races.

6) Could we implement Send ourselves using safe rust? why/why not?

No, implementing Send manually in safe Rust is generally not possible. This is because Send implementations must ensure that the type can be safely moved between threads without causing data races or memory issues. Ensuring thread safety usually requires access to unsafe blocks, especially when handling types that involve raw pointers or low-level concurrency control. Rust’s strict checks around Send help prevent unintentional thread-unsafe behaviors that could compromise program safety.