
// YOUR MACRO HERE
macro_rules! avg {
    ($($n:expr), *) => {
        {
            let mut sum = 0;
            let mut nums = 0;
            $(
                sum += $n;
                nums += 1;
            )*
            sum / nums
        }
    };
}

// DO NOT CHANGE
fn main() {
    let a = avg!(1, 2, 3, 4, 5);
    println!("a = {}", a);

    assert_eq!(a, 3);

    let b = avg!(a, 10, 20);
    println!("b = {}", b);
    assert_eq!(b, 11);
}
