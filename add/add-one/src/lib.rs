use rand;
pub fn add_one_fn(a:u32)->u32{
    println!("{}", a);
    a + 2 
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result =  add_one_fn(3);
        assert_eq!(result, 5);
    }
}
