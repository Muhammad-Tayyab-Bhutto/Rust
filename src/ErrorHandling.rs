// // A function that divides two numbers and handles division by zero
// fn divide(a: f64, b: f64) -> Result<f64, String> {
//     if b == 0.0 {
//         Err(String::from("Division by zero!"))
//     } else {
//         Ok(a / b)
//     }
// }

// fn main() {
//     // Call the divide function
//     match divide(4.0, 0.0) {
//         Ok(result) => println!("Result: {}", result),
//         Err(e) => println!("Error: {}", e),
//     }

//     match divide(4.0, 2.0) {
//         Ok(result) => println!("Result: {}", result),
//         Err(e) => println!("Error: {}", e),
//     }
// }


// fn find_char_in_string(s: &str, c: char) -> Option<usize> {
//     s.find(c)
// }

// fn main() {
//     let my_str = "Hello, world!";
    
//     match find_char_in_string(my_str, 'w') {
//         Some(index) => println!("Found character at index: {}", index),
//         None => println!("Character not found!"),
//     }

//     match find_char_in_string(my_str, 'z') {
//         Some(index) => println!("Found character at index: {}", index),
//         None => println!("Character not found!"),
//     }
// }

// fn main() {
//     let result: Result<i32, &str> = Ok(10);
//     println!("Value: {}", result.unwrap()); // Works fine
    
//     let result: Result<i32, &str> = Err("Oops, error occurred");
//     println!("Value: {}", result.unwrap()); // Panics here
// }


// use std::fs::File;
// use std::io::{self, Read};

// fn read_file_content() -> Result<String, io::Error> {
//     let mut file = File::open("hello.txt")?; // If File::open fails, it returns an error
//     let mut content = String::new();
//     file.read_to_string(&mut content)?; // Propagates the error if read_to_string fails
//     Ok(content)
// }

// fn main() {
//     match read_file_content() {
//         Ok(content) => println!("File content: {}", content),
//         Err(e) => println!("Error: {}", e),
//     }
// }

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

