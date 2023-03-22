fn main() {
    println!("Hello, world!");
}

pub fn sum_bool(a:u32, b:u32) -> bool {
    if a + b == 4 { return true} else {return false}
}



#[cfg(test)]
mod test {
    use super::* ;

    #[test]
    fn testing() {
        assert_ne!(2+2, 4, "enter other than 2 + 2 = {}", 4)
    }

    #[should_panic(expected="enter 2 + 2")]
    #[test]
    fn test_sum_bool() {
        assert!(sum_bool(22,14), "enter 2 + 2")
    }

    #[test]
    #[ignore]
    fn test_using_res() -> Result<(), String>{
        if 2 + 2 == 5 {
            Ok(())
        }
        else {
            Err(String::from("You are wrong"))
        }
    }
}