//Pattern 1 to capture error

fn main() {
    // Simulating a function that can return a Result
    fn whatever_function(y: i32) -> Result<i32, String> {
        //#1 The main boiler template code ( inject this to end of begging of your function)
        if y == 0 {
            return Err("you really should not use zero".to_string());
        }
        let return_success_result: i32 = 100;
        Ok(return_success_result)
        //end #1
    }

    //example of capture error when you want to exist from program when it fails
    fn exit_when_fails_function(some_value: i32) -> i32 {
        let return_success_result: i32 = 100;
        if some_value == 0 {
            panic!("sorry bad value in we are exit the program now");
        }

        println!("value is {}", return_success_result);
        return return_success_result;
    }

    // Attempt to perform division
    let result = whatever_function(0);

    //#2 The main boiler template code , this is where we capture error
    // Use unwrap_or_else to handle the error case
    let value = result.unwrap_or_else(|err| {
        println!("Error: {}", err);
        // You can provide custom error-handling logic here
        // For example, returning a default value
        10
    });
    //#2

    println!("Result: {}", value);
    exit_when_fails_function(0);
}
