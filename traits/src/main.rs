mod greeting;
mod person;

use person::Person;
use greeting::Greeting;

fn main() {
    let person = Person { name: "Alice".to_string() };
    person.greet();
}
