pub trait Draw {
    fn draw(&self) {
        println!("default drawing mode");
    }
}


pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw()
        }
    }
}

pub struct Button {
    pub height: u32,
    pub width: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("button is created");
    }
}

pub struct Select {
    pub height: u32,
    pub width: u32,
    pub options: Vec<String>,
}

impl Draw for Select {
    fn draw(&self) {
        println!("select is created");
    }
}


