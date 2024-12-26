fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest number is {result}");

    let _int_point = Point { _x: 5, _y: 10 };
    let _float_point = Point { _x: 1.0, _y: 4.0 };
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
