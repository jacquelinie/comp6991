fn main() {
    let mut vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let a = &mut vec;

    a.push(11);
    a.push(12);
}

// Vec was borrowed as mutable more than once. (let a = &mut vec AND let b = &mut vec)
// You cannot borrow `vec` as mutable more than once at a time.
// In Rust, you can either have many immutable references, or one mutable reference.
// Thus by violating this rule, the program cannot compile.