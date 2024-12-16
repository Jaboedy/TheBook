fn main() {
    let mut x = five();
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let spaces = "   ";
    let spaces = spaces.len();

    println!("The spaces variable is this long: {spaces}");
    // let mut spaces = "   "; DOES NOT WORK
    // spaces = spaces.len();
}

fn five() -> i32 {
    5
}
