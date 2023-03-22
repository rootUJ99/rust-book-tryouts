pub trait Abstract {
    fn show_abstract(&self) -> String {
        String::from("this is default abstract")
    }
}

#[derive(Debug)]
pub struct Languages {
    pub name: String,
    pub country: String,
}

#[derive(Debug)]
pub struct ProgrammingLangs {
    pub name: String,
    pub original_author: String,
}

impl Abstract for Languages {
    // fn show_abstract(&self) -> String {
    //     format!("{} is spoken most in {}", self.name, self.country)
    // }
}

impl Abstract for ProgrammingLangs {
    fn show_abstract(&self) -> String {
        format!("the author of {} is {}", self.name, self.original_author)
    }
} 

