fn main() {
    let number: u32 = 10;
    let mut sum: u32 = 2;
    let mut prev: u32 = 1;
    let mut count: u32 = 0;
    while count != number {
        if count == 0 {
            println!("{}\n{}", prev, sum);
        }
        sum = sum + prev;
        prev = sum - prev;
        count = count + 1;
        println!("{}",sum);
    }
}
