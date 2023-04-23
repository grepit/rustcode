#![allow(unused)]
fn main() {
pub struct Guess {
    value: i32,
}


impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

  let guess = Guess::new(50000);
    println!("Guess value is {}.", guess.value());
// let here : i32=100;
// let guess= Guess(
//     here,
//     );

}