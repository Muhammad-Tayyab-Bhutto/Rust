struct User<'a, 'b> {
    name: &'a str,
    age: &'b u8,
}
fn main() {
    let user: User;
    let name = String::from("Muhammad Tayyab Bhutto");
    {
        let age: u8 = 22;
        user = User{name: &name, age: &age};
        println!("Name: {}, Age: {}", user.name, user.age);
    }
    // println!("Name: {}, Age: {}", user.name, user.age);    // error: borrowed value does not live long enough
}