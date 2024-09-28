fn main() {
    // variables
    let x: i8 = 10;
    let y: i8 = 20;
    let is_even: bool;
    let greetings: String = String::from("This is Muhammad Tayyab");
    let character: char = 'a';
    print!("x + y = {}\n", x + y);
    if x%2 == 0 {
        is_even = true;
    }
    else {
        is_even = false;
    }
    print!("Is x even? {}\n", is_even);
    println!("{}", greetings);
    print!("{}", character)
}
