use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("test.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("test.txt") {
                Ok(fe) => fe,
                Err(er) => panic!("{:?}", er),
            }
            other_error => {
                panic!("{:?}", other_error)
            }
        },
    };
}
