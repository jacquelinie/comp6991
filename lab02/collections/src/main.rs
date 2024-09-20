use std::collections::VecDeque;
use std::collections::LinkedList;
use std::collections::HashMap;

const MAX_ITER: i32 = 300000;

fn main() {
    // Vectors
    vec_operations();

    // VecDeque
    vec_deque_operations();

    // Linked List
    linked_list_operations();

    // Hashmap
    hashmap_operations();

    // Times:
    // ==== Vector ====
    // insert: 4.994497ms
    // remove: 3.782154973s
    // ==== VecDeque ====
    // insert: 8.434411ms
    // remove: 9.933261ms
    // ==== LinkedList ====
    // insert: 39.26991ms
    // remove: 18.191835ms
    // ==== HashMap ====
    // insert: 266.947872ms
    // remove: 151.231306ms


    /* Questions
    - Which collection type was the fastest for adding and removing elements?
    VecDeque's adding is the fastest and Vector's removing is the fastest.

    - Why do you think this was the case?
    VecDeque's adding is fastest because of it uses a ring buffer data structure. This means that when adding elements,
    VecDeque can use existing memory space and there is no need for frequent memory allocation and copy operations unlike the other data structures.
    Vector's removing is the fastest because when an element is removed from Vector, it fills the void by moving the element out of memory.

    - Is there any significant difference between Vec and VecDeque deletion?
    Yes there is, Vector being 3.78s while VecDeque being 9.93s.

    - If so, why? If not, why not?
    When a Vector deletes elements, it uses the CPU cache to reduce the number of memory accesses and thus improves efficiency.

    - When would you consider using VecDeque over Vec?
    Since VecDeque uses a ring buffer to store data, it would be suitable for situations
    where the operation of adding and removing from the front or the end is done often.

    - When would you consider using LinkedList over Vec?
    Since LinkedList uses dynamic memory allocation, it would be suitable for situations where
    inserting or deleting elements from the middle of the list is done often.

    - Did the results suprise you? Why or why not?.
    I was surprised by the speed of insertion and removal of HashMap. Since in other languages it would be one of the fastest and
    theoretically should have the lowest time complexity for its operations. However in Rust, HashMaps use hash buckets and chains
    which makes it slower than other data structures.
    */

}

/// measure the insertion and removal
/// operations of a vector
fn vec_operations() {
    let mut vec = Vec::new();

    let time_start = std::time::Instant::now();
    for i in 0..MAX_ITER {
        vec.push(i);
    }
    let time_end = std::time::Instant::now();

    println!("==== Vector ====");
    println!("insert: {:?}", time_end - time_start);

    let time_start = std::time::Instant::now();
    for _ in 0..MAX_ITER {
        vec.remove(0);
    }
    let time_end = std::time::Instant::now();

    println!("remove: {:?}", time_end - time_start);
}

/// measure the insertion and removal
/// operations of a VecDeque
fn vec_deque_operations() {
    let mut vec_deque = VecDeque::new();

    let time_start = std::time::Instant::now();
    for i in 0..MAX_ITER {
        vec_deque.push_back(i);
    }
    let time_end = std::time::Instant::now();

    println!("==== VecDeque ====");
    println!("insert: {:?}", time_end - time_start);

    let time_start = std::time::Instant::now();
    for _ in 0..MAX_ITER {
        vec_deque.pop_front();
    }
    let time_end = std::time::Instant::now();

    println!("remove: {:?}", time_end - time_start);
}


// Operations of Linked list
fn linked_list_operations() {
    let mut linked_list = LinkedList::new();

    let time_start = std::time::Instant::now();
    for i in 0..MAX_ITER {
        // Add Operation
        linked_list.push_back(i);
    }
    let time_end = std::time::Instant::now();

    println!("==== LinkedList ====");
    println!("insert: {:?}", time_end - time_start);

    let time_start = std::time::Instant::now();
    for _ in 0..MAX_ITER {
        // Remove Operation
        linked_list.pop_front();
    }
    let time_end = std::time::Instant::now();

    println!("remove: {:?}", time_end - time_start);
}

// Operations of Hash map
fn hashmap_operations() {
    let mut hashmap = HashMap::new();

    let time_start = std::time::Instant::now();
    for i in 0..MAX_ITER {
        // Add Operation
        hashmap.insert(i, i);
    }
    let time_end = std::time::Instant::now();

    println!("==== HashMap ====");
    println!("insert: {:?}", time_end - time_start);

    let time_start = std::time::Instant::now();
    for i in 0..MAX_ITER {
        // Remove Operation
        hashmap.remove(&i);
    }
    let time_end = std::time::Instant::now();

    println!("remove: {:?}", time_end - time_start);
}