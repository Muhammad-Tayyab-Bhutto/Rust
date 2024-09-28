pub trait Summary {
    fn summarise(&self) -> String {
        return String::from("This is a summary trait default message!")
    }
}

struct User {
    name: String,
    age: u32,
}

trait Message {
    fn message(&self) -> String {
        return String::from("This is a message trait default message!")
    }
}

struct Worker {
    name: String,
    role: String,
    salary: f64,
}

impl Summary for User {
    fn summarise(&self) -> String {
        return format!("name: {} role: {} salary: {}", self.name, self.role, self.salary);
    } 
}

impl Message for User {
    fn summarise(&self) -> String {
        return format!("name: {} role: {}", self.name, self.role);
    }
}
impl Message for Worker {
    fn summarise(&self) -> String {
        return format!("name: {} role: {}", self.name, self.role);
    }
}
// Trait as Parameter: Iska mtlb y hy srif wahi is ko parameters ja sakty hn jo trait ko impl kr rahy ho jo nhi krty wo nhi ja sakty for intstance String, u32, ... etc...
// fn notify(u: impl Summary) {
//     println!("{}", u.summarise());
// }

fn notify<T: Summary + Message> (u: T) {
    println!("{}", u.summarise());
}

fn main() {
    let user: User = User{name: String::from("Muhammad Tayyab Bhutto"), age: 22};
    println!("{}", user.summarise());
    // notify(user); // Trait as a parameter
    let worker: Worker = Worker{name: String::from("John Doe"), role: String::from("Software Engineer"), salary: 50000.0};
    println!("{}", worker.summarise());
    // notify(worker); // Trait as a parameter
    notify(user); // Trait as a parameter
    // notify(worker); // Trait as a parameter
}


