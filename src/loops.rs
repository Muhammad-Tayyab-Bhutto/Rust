// Loop: An infinite loop until manually stopped with break.
// While Loop: Loops while a condition is true.
// For Loop: Iterates over a range or collection.
fn main() {
    // Infinite loop
    let mut count = 0;
    loop {
        count += 1;
        if count == 5 {
            println!("Count is 5, breaking the loop!");
            break;
        }
    }

    // While loop
    let mut n = 3;
    while n > 0 {
        println!("{}!", n);
        n -= 1;
    }

    // For loop (over a range)
    for i in 1..4 {
        println!("i = {}", i);
    }

    // For loop (over an array)
    let arr = [10, 20, 30, 40];
    for elem in arr.iter() {
        println!("Value in array: {}", elem);
    }
}
