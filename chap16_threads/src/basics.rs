// not realted to threads 

use std::fmt::{Display, Debug};
use std::marker::Sized;

fn print_ln<T: Display + Debug + Sized>(s: [T]) -> T {
    println!("{:?}", s);
    s[0]
}