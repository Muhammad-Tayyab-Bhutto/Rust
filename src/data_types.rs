fn main() {
    // Integer
    let _x: i32 = 42;

    // Float
    let _y: f64 = 3.14;

    // Boolean
    let _is_active: bool = true;

    // Character
    let _letter: char = 'A';

    // Tuple
    let tuple: (i32, f64, char) = (100, 6.4, 'Z');
    
    // Access tuple values
    println!("The first value of the tuple is: {}", tuple.0);

    // Array
    let arr: [i32; 4] = [1, 2, 3, 4];
    println!("The second value of the array is: {}", arr[1]);
}
