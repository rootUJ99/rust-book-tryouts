
use std::fmt::Display;

pub fn largest<T: PartialOrd+Copy+Display>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    println!("-----------{}",largest);
    largest
}

