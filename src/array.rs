fn main() {
    // Define an array
    let numbers: [i32; 4] = [1, 2, 3, 4];

    // Access elements in an array
    println!("The first element is: {}", numbers[0]);

    // Loop through array elements
    for number in numbers.iter() {
        println!("Array element: {}", number);
    }

    // Array length
    println!("The array length is: {}", numbers.len());
}
