use std::io;

fn farenheit_to_celcius(temp_in_fr: f32)-> f32{
    (temp_in_fr - 32 as f32) * 5 as f32 / 9 as f32
}

fn celcius_to_farenheit(temp_in_celcius: f32) -> f32 {
    temp_in_celcius * (9 as f32 / 5 as f32) + 32 as f32
}

fn main() {
    let mut key = String::new();
    println!("print 0 for cel to fr or print any key for fr to cel");
    io::stdin()
        .read_line(&mut key)
        .expect(" faild to read the line");

    let mut input = String::new();
    println!("enter the temprature");
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read the line");

    let key: u32 = key.trim().parse().expect("please type a valid number");

    let input: u32 = input.trim().parse().expect("please type a valid number");

    let converted_value = if key == 0 {celcius_to_farenheit(input as f32)} else {farenheit_to_celcius(input as f32)};
    println!("{}",converted_value)
}
