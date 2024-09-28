// write a logic to filter all the odd values then double each value and create a new vector

fn main() {
    let vector: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let even_numbers = vector.iter().filter(|&x| x % 2 == 0).map(|x| x * 2);

    println!("{:?}", even_numbers.collect::<Vec<_>>());  // Collecting the iterator into a Vec to print it
}

