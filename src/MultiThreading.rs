use std::thread;

fn main() {
    let v: Vec<i32> = vec![1, 2, 3];
    // move the ownership of v to the spawned thread
    let _handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
        thread::sleep(std::time::Duration::from_millis(1));
    });
    // let _handle = thread::spawn( || {
    //     for i in 1..10 {
    //         println!("hi number {} from the spawned thread!", i);
    //         // thread::sleep(std::time::Duration::from_millis(100));
    //     }
    // });
    // let _handle = thread::spawn(move || {
    //     for i in 1..10 {
    //         println!("hi number {} from the spawned thread!", i);
    //         // thread::sleep(std::time::Duration::from_millis(100));
    //     }
    // });
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(std::time::Duration::from_millis(100));
    }
}