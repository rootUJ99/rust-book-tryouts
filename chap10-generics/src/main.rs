mod exploring_traits;
mod trait_bound;
mod lifetimes;
use exploring_traits::{Abstract, Languages, ProgrammingLangs};
use trait_bound::largest;
use lifetimes::greater;

#[derive(Debug)]
struct Address<T, U> {
    pin: U,
    state: T,
    country: T,
}

impl<T, U> Address<T, U> {
    fn show_pin(&self) -> &U {
        &self.pin
    }
    fn show_state(&self) -> &T {
        &self.state
    }
    // fn full_address<W: Display>(&self) -> W {
    //     format!("{} {} {}", &self.state, self.country, self.pin)
    // }
}

fn main() {
    let add = Address {
        state: String::from("Maharashtra"),
        country: String::from("India"),
        pin: 400101,
    };
    let state_name = add.show_state();
    let pincode = add.show_pin();
    println!("{:?} {} {}", add, state_name, pincode);


    let marathi = Languages {
        name: String::from("Marathi"),
        country: String::from("India"),
    };
    let marathi_abstract = marathi.show_abstract();
    println!("{:?} {}", marathi, marathi_abstract);

    let javascript = ProgrammingLangs {
        name: String::from("JavaScript"),
        original_author: String::from("Brendan Eich"),
    };
    let javascript_abstract = javascript.show_abstract();
    println!("{:?} {}", javascript, javascript_abstract);

    let number_list = vec![5,55,6,3,6,8];
    let string_list = vec!['z', 'm', 'p', 'a', 'u'];

    let largest_num = largest(&number_list);
    let largest_char = largest(&string_list);

    println!("number {} character {}", largest_num, largest_char);

    let x = "what you want";
    let y = "what you expect";

    let greatest = greater(&x, &y, "this works baby");
    println!("greatest {} ", greatest);
}
