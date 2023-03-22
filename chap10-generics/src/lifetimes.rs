use std::fmt::Display;

pub fn greater<'a, T>(
    x: &'a str, 
    y: &'a str,
    ann: T
) -> &'a str
where T: Display
{
    println!("Okay this works {}", ann);
    if x.len() > y.len() {
        return x
    } 
    y
}