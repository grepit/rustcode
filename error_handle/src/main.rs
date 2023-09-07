enum MyError {
    Error {
        message: String,
    },
}

//Example check for integer and throw an exception
fn divide(x: i32, y: i32) -> Result<i32, MyError> {
    if y == 0 {
        return Err(MyError::Error { message: "generic erro by zero".to_string() });
    }

    Ok(x / y)
}

//Example check for string 
fn string_wild(string: &str)  -> Result<usize, String> {
    if string.is_empty() {
        return Err("The string is empty.".to_string());
    }
    else if string.contains("hi"){
         return Err("The string is not suppose to have hi.".to_string());
    }

    return Ok(string.len());
}


fn main() {
    let result = divide(10, 0);

    match result {
        Ok(value) => println!("The result is {}", value),
        Err(MyError::Error { message }) => {
            println!("An error occurred: {}", message);
        }
    }



let mystring = "Hello hi, world!";
let length = string_wild(&mystring);

match length {
    Ok(value) => println!("The length of the string is {}.", value),
    Err(error) => println!("An error occurred: {}", error),
}


}






/*
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
*/

