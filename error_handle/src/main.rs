
fn main() {
    
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err(String::from("Cannot divide by zero!"));
    }
    Ok(a / b)
}



    let dividend = 10;
    let divisor = 2;

    // Using Option
    let result_option = dividend.checked_div(divisor);
    match result_option {
        Some(result) => println!("Result using Option: {}", result),
        None => println!("Cannot divide by zero using Option!"),
    }

    // Using Result
    let result_result = divide(dividend, divisor);
    match result_result {
        Ok(result) => println!("Result using Result: {}", result),
        Err(err) => println!("Error using Result: {}", err),
    }
}