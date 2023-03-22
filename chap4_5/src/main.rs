// fn borrow_values(some_string: &mut String) -> String {
//     some_string.push_str("oh yeah i got something like this");
//     some_string.to_uppercase()
// }
#[derive(Debug)]
struct Info {
    name: String,
    age: u32,
    is_programmer: bool,
}

impl Info {
    fn handle_is_programmer (&self) {
        if self.is_programmer == true { println!("yes he is")} else {println!("nope")}
    }

    fn handle_predict_single(&self) -> bool{
        self.is_programmer == true && self.age < 30
    }
}

fn handle_college_student(info: &Info) -> bool{ 
    info.is_programmer == true && info.age < 22
}

fn main() {
    
    let user = Info {
        name: String::from("Ujwal Arak"),
        age: 25,
        is_programmer: true,
    };

    println!("{:#?}", user);
    user.handle_is_programmer();
    let is_single = user.handle_predict_single();
    println!("he is single {}", is_single);

    // let mut value = String::from("something like this");
    // let value_from_fn = borrow_values(&mut value);
    // println!("{} {}", value, value_from_fn);
    let is_student = handle_college_student(&user);
    println!("he is college student {}", is_student);
}
