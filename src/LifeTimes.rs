// fn logest_string(s1: String, s2: String) -> String {
//     if s1.len() > s2.len() {
//         return s1
//     }
//     else {
//         return s2
//     }
// }   
fn main() {
    let s1: String = String::from("hello");
    {
        let s2: String = String::from("world");
        println!("The longest string is: {}", logest_string(&s1, &s2));
    }
    // println!("The longest string is: {}", logest_string(&s1, &s2)); // Scope error due to s2 
}

fn logest_string<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        return s1
    }
    else {
        return s2
    }
}