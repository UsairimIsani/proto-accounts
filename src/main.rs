fn main() {
    // Closures in Rust
    //
    // basic_eg();
    eg_two();
}
fn eg_two() {
    let a = vec![1, 2, 3];
    let b = vec![1, 2, 3];
    let s_c = move |x: Vec<i32>, y: Vec<i32>| {
        let z: Vec<_> = x.iter().zip(y.iter()).collect();
        println!("{:?}", z);
        // [(1, 1), (2, 2), (3, 3)]
    };
    s_c(a, b);
    // println!("{:?}",a); // Wont work a was moved inside the closure so was dropped
    let some = vec![1, 2, 3, 4, 5];
    let other: Vec<_> = some.iter().map(|x| x + x).map(|y| y - 1).collect();
    println!("{:?}", other);
    // [1, 3, 5, 7, 9]
}

fn basic_eg() {
    let y = 2;
    let x = 2;
    let some_closure = |x| x == y;
    println!("is x :{} equal to y : {} ? {} ", x, y, some_closure(x));
}
