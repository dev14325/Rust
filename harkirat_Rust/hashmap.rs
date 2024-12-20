use std::collections::HashMap;

fn main(){
    let mut user = HashMap::new();
    user.insert(String::from("abc"),22);
    user.insert(String::from("def"),32);
    let first_user_age = user.get("abc"); // return option
    match first_user_age {
        Some(val) =>println!("{}",val),
        None => println!("user not found"),
    }
    
}