use std::collections::HashMap;
fn main() {
    let mut users = HashMap::new();
    users.insert(String::from("sachin"), 22);
    users.insert(String::from("saurav"), 32);

    let first_user_age = users.get("saurav");
    match first_user_age{
        Some(age) => println!("age is {}",age),
        None => println!("user not found"),
    }
}





