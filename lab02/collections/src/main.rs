use std::collections::VecDeque;

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


    // Questions
    // Which collection type was the fastest for adding and removing elements?
    // Why do you think this was the case?
    // Is there any significant difference between Vec and VecDeque deletion?
    // If so, why? If not, why not?
    // When would you consider using VecDeque over Vec?
    // When would you consider using LinkedList over Vec?
    // Did the results suprise you? Why or why not?.
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
        linked_list.push_back(i);
    }
    let time_end = std::time::Instant::now();

    println!("==== LinkedList ====");
    println!("insert: {:?}", time_end - time_start);

    let time_start = std::time::Instant::now();
    for _ in 0..MAX_ITER {
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
        hashmap.insert(i, i);
    }
    let time_end = std::time::Instant::now();

    println!("==== HashMap ====");
    println!("insert: {:?}", time_end - time_start);

    let time_start = std::time::Instant::now();
    for i in 0..MAX_ITER {
        hashmap.remove(&i);
    }
    let time_end = std::time::Instant::now();

    println!("remove: {:?}", time_end - time_start);
}