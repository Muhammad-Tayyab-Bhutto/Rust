fn main() {
    let number = 7;

    if number < 10 {
        println!("The number is less than 10");
    } else if number == 10 {
        println!("The number is exactly 10");
    } else {
        println!("The number is greater than 10");
    }

    // Ternary-like expression
    let is_even = if number % 2 == 0 { "even" } else { "odd" };
    println!("The number is {}", is_even);
}
