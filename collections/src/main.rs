use std::collections::HashMap;

fn main() {
    vectors();
    strings();
    hashmaps();
}

fn hashmaps() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("Blue Score: {score}");

    // iterating over key-value pairs
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // putting non Copy values into a hashmap moves them
    // we can instead insert references, but the references must remain valid

    // Entries can be overwritten
    scores.insert(String::from("Blue"), 15);
    scores.insert(String::from("Blue"), 25);
    println!("{scores:?}");

    // Insert only if a key doesn't exist
    scores.entry(String::from("Purple")).or_insert(45);
    scores.entry(String::from("Blue")).or_insert(3);
    println!("{scores:?}");

    // update based on old value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{map:?}");

    // Hashmap uses SipHash hashing alg by default
    // This can be changed if desired by specifying
    // a different hasher (type that implements BuildHasher trait)
}

fn strings() {
    let mut s = String::new();
    println!("s: {s}");

    let data = "initial contents";
    s = data.to_string();
    println!("s: {s}");
    s = "next contents".to_string();
    println!("s: {s}");

    //equivalent option
    s = String::from("3rd contents");
    println!("s: {s}");

    s = String::from("foo");
    s.push_str("bar");
    println!("s: {s}");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s1: {s1}");
    println!("s2: {s2}");

    let mut s = String::from("lo");
    s.push('l');
    println!("s2: {s2}");

    //concatenation with + or format! macro
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // this uses the add(self, &str) -> String method
    println!("s3: {s3}"); // s1 has been moved here, s2 has not
                          // this works even though s2 is &String, not &str
                          // because of automatic deref coercion from &s2 to &s2[..]
    println!(":)");

    // when concatenating multiple strings the + operator gets ugly
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("s: {s}");

    // use format! instead
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}"); //doesn't take ownership of any params
    println!("s: {s}");

    // Rust does not support string indexing
    // s[0] does not work
    // However, you can slice a string like so
    // which returns the first 4 bytes of "hello"
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("s: {s}");

    // iterating over characters in a String
    for c in s.chars() {
        println!("{c}");
    }

    // iterating over bytes in a String
    for b in s.bytes() {
        println!("{b}");
    }
}

fn vectors() {
    let _v: Vec<i32> = Vec::new();
    let mut v2 = vec![1, 2, 3];
    v2.push(4);
    v2.push(5);
    v2.push(6);
    println!("{:?}", v2);

    let third: &i32 = &v2[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v2.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }

    // iterating over a vec and changing values
    for i in &mut v2 {
        *i += 50;
    }
    println!("{v2:?}");

    // use an enum to store multiple types
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let mut row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    row.push(SpreadsheetCell::Int(5));
    println!("{row:?}");
}
