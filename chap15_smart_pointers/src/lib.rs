pub fn add_one(x:u32) -> u32{
    x + 20
}

mod refcell_pointer;

#[cfg(test)]
mod tests {
    use refcell_pointer::*;
    use super::*;
    use std::cell::RefCell;

    struct MockMessanger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessanger {
        fn new()-> MockMessanger {
            MockMessanger {
                sent_messages: RefCell::new(vec![])
            }
        }
    }

    impl Messanger for MockMessanger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message))
        }
    }


    #[test]
    fn send_75_percent_quota() {
        let mockmessanger = MockMessanger::new();
        let mut limittracker = LimitTracker::new(&mockmessanger, 100);

        limittracker.set_value(80);
        
        let len = mockmessanger.sent_messages.borrow().len();
        println!("{}", len);
        assert_eq!(len ,1);

    }

}