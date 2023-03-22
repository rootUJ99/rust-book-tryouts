mod unsafe_rust;
use unsafe_rust::mighty_fn;
use std::slice;
use std::ops::Add;
use std::fmt;


static mut COUNTER: u32 = 0;

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len-mid),
        )
    }
}

fn add_to_count(inc: u32){
    unsafe {
        COUNTER += inc
    }
}

trait OulinePoint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Point {
    x: u32,
    y: u32,
}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point{
        Point {
            x: self.x + other.x, 
            y: self.y + other.y, 
        }
    }
}

impl OulinePoint for Point {}

impl fmt::Display for Point{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} {})", self.x, self.y)
    }
}

#[derive(Debug)]
struct Milimeters(u32);
struct Meters(u32);

impl Add<Meters> for Milimeters {
    type Output = Milimeters;

    fn add(self, other:Meters) -> Milimeters {
        Milimeters(self.0 + (other.0 * 1000))
    }
}

trait Wizard {
    fn fly();
}

trait Pilot {
    fn fly(&self);
}

struct Human;

impl Wizard for Human {
    fn fly() {
        println!("flying on a brooom?");
    }
}

impl Pilot for Human {
    fn fly(&self) {
        println!("flying a plane");
    }
}

impl Human {
    fn fly(&self) {
        println!("Huhh i cant fly");
    }
}

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.join(", "))
    }
}

fn main() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is :{}" , *r1);
        println!("r2 is :{}" , *r2);
    }

    unsafe { 
        let num = mighty_fn();
        println!("{}", num);

    }
    let mut array : [i32; 6] = [2,4,5,6,2,4];
    let slice = &mut array[..];
    split_at_mut(slice, 2);

    add_to_count(22);

    unsafe {println!("{}", COUNTER); }

    let a = Point {
        x: 22,
        y: 33,
    };

    let b = Point {
        x: 22,
        y: 33,
    };

    let c = a + b;
    println!("{:?}", c);
    assert_eq!(a+b,c);

    let mtr = Meters(7);
    let milmtr = Milimeters(10);

    let mtrmilmtr = milmtr + mtr;

    println!("{:?}", mtrmilmtr);

    let human = Human;
    human.fly();
    <Human as Wizard>::fly();
    Pilot::fly(&human);

    let w = Wrapper(vec![String::from("hello"), String::from("mello")]);

    println!("w -> {}", w);

    type Kilo = u32;

    let kilo: Kilo = 22;

    println!("{}", kilo);

}


