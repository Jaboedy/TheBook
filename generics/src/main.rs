use std::fmt::{Debug, Display};

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest number is {result}");

    let _int_point = Point { _x: 5, _y: 10 };
    let _float_point = Point { _x: 1.0, _y: 4.0 };
    summary_test();
}

// abstract the functionality to avoid duplication
// then we specify that for any type T vec passed in
// an item of type T will be returned
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// We can also use generics in struct definitions
struct Point<T> {
    _x: T,
    _y: T,
}

// Implementing methods with generics
impl<T> Point<T> {
    fn _x(&self) -> &T {
        &self._x
    }
}

// Specifying constraints on generic types for a method
impl Point<f32> {
    fn _distance_from_origin(&self) -> f32 {
        (self._x.powi(2) + self._y.powi(2)).sqrt()
    }
}

// To define a struct where values could have different
// generic types do the following
struct _Point2<T, U> {
    _x: T,
    _y: U,
}

// Same goes for enums
enum _Option<T> {
    Some(T),
    None,
}

enum _Result<T, E> {
    Ok(T),
    Err(E),
}

// Traits are similar to a feature called interfaces
// in many other languages
pub trait Summary {
    fn summarize(&self) -> String;

    // or default behaviors can be provided
    fn summarize_with_default(&self) -> String {
        String::from("(Read more...)")
    }
    // these methods with defaults can call other
    // methods defined in the trait
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn summary_test() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("Of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}

// To specify a function should accept a parameter
// only if it is a type that implements a trait
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// This can also be written as follows
pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// To implement more than one required
// trait you can use this syntax
pub fn notify3(item: &(impl Summary + Display)) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify4<T: Summary + Display>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Trait bounds can get complicated, where clauses
// can help clarify this
pub fn some_function<T: Display + Clone, U: Clone + Debug>(_t: &T, _u: &U) -> i32 {
    println!("Demo");
    1
}

pub fn some_alternative_function<T, U>(_t: &T, _u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    println!("Demo");
    1
}

// We can also use the impl Trait syntax to specify
// a return type.  This syntax doesn't work if
// multiple types could be returned depending on a condition
// in the function.  This case will be covered later
pub fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("name"),
        content: String::from("content"),
        reply: false,
        retweet: false,
    }
}

// You can also use trait bounds to implement
// methods on a type only if it implements other
// traits
struct _Pair<T> {
    x: T,
    y: T,
}

impl<T> _Pair<T> {
    fn _new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> _Pair<T> {
    fn _cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// We can also conditionally impl a Trait for any type that
// implements another trait.  This is called a blanket implementation
// impl<T: Display> ToString for T {
//    // --snip--
// }
