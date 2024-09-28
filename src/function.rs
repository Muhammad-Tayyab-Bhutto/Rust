fn main() {
    let result = add(5, 10);
    println!("The result is: {}", result);

    greet("Alice");
}

// Function that returns the sum of two integers
fn add(x: i32, y: i32) -> i32 {
    x + y  // No need for `return` in the last expression
}

// Function that takes a string and prints a greeting
fn greet(name: &str) {
    println!("Hello, {}!", name);
}
