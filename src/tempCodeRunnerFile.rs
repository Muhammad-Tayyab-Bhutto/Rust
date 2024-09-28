use std::fmt;

// Define a custom error type
#[derive(Debug)]
enum MyError {
    DivisionByZero,
    NegativeNumber,
}

// Implement the `fmt::Display` trait to provide a user-friendly error message
impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MyError::DivisionByZero => write!(f, "Cannot divide by zero"),
            MyError::NegativeNumber => write!(f, "Negative number encountered"),
        }
    }
}

// A function that uses the custom error type
fn custom_divide(a: f64, b: f64) -> Result<f64, MyError> {
    if b == 0.0 {
        Err(MyError::DivisionByZero)
    } else if a < 0.0 {
        Err(MyError::NegativeNumber)
    } else {
        Ok(a / b)
    }
}

fn main() {
    match custom_divide(-4.0, 2.0) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}