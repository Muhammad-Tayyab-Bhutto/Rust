use std::collections::HashMap;

fn main() {
    let name = String:form("Muhammad Tayab");
    let thisName = name;
    print!("{}", borrow_value(&name));
    print!("{}", name);
    print!("{}", name);
}

fn group_values_by_keys(vec: Vec<(String, i32)>) -> HashMap<String, i32> {
    let mut result: HashMap<String, i32> = HashMap::new();
    
    for (key, value) in vec {
        result.insert(key, value);
    }   
    return result;
}

fn borrow_value(name: &str) -> String {
    let nam = name;
    return nam.to_string();
}