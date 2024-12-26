use std::error::Error;
use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

fn main() {
    // To purposely cause a panic
    // panic!("Crash and Burn")

    // this would cause a panic for trying to access
    // an index outside of the range v posesses

    // let mut v = vec![1, 2, 3];
    // v.push(4);
    // let x = v[99];
    // println!("The value of x is: {x}");
    let greeting_file_result = File::open("hello.txt");
    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    };
    // the unwrap method on the Result<T, E> type
    // will return the value inside of Ok variants
    // and call panic! macro for us with Err variants
    let _greeting_file = File::open("hello.txt").unwrap();

    // similarly the expect method lets us choose the
    // panic! error message.
    let _greeting_file =
        File::open("hello.txt").expect("hello.txt should be included in this project");
}

// Propagating Errors Manually
fn _read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// Propagating errors with the ? Operator
fn _read_username_from_file2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// Alternative (and even shorter) implementation
fn _read_username_from_file3() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

// Shortest implementation
// reading a file into a string is very common
// and the standard library already provides this
// capability
fn _read_username_from_file4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// The ? operator can only be used in functions
// whose return type is compatible with the value
// the ? is used on
//
// This must be a return type of Result, Option
// or any other type that implements FromResidual
// Whichever the calling function returns

fn _last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

// The main function is required to have particular return
// types, lucily the type Result<(), E> is included
// so the following would work
//
// fn main() -> Result<(), Box<dyn Error>> {
//   let greeting_file = File::open("hello.txt")?;
//   Ok(())
// }
//
// here Box<dyn Error> is a Trait object which is talked
// about more in another unit, but for now think of it as
// any type of error
