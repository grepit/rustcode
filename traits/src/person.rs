use crate::greeting::Greeting;

pub struct Person {
    pub name: String,
}

impl Greeting for Person {
    fn greet(&self) {
        println!("Hello, my name is {}.", self.name);
    }
}
