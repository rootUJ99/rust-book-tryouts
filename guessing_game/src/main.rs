use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let rand = rand::thread_rng().gen_range(1..10);
    // println!("{}",rand);
    println!("Guess the number!");
    println!("please input your number");
    
    loop {

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
        
        match guess.cmp(&rand) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too large"),
            Ordering::Equal => {
                println!("---You WON!!!---");
                break;
            },
        }
    }
}
