fn main() {
    // To purposely cause a panic
    // panic!("Crash and Burn")

    let mut v = vec![1, 2, 3];
    v.push(4);
    let x = v[99];
    println!("The value of x is: {x}");
}
