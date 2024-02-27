fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return String::new();
    }

    let mut prefix = String::new();
    
    for i in 0..strs.iter().map(|s| s.len()).min().unwrap() {
        let c = strs[0].as_bytes()[i];
        if strs.iter().all(|s| s.as_bytes()[i] == c) {
            prefix.push(c as char);
        } else {
            break;
        }
    }

    prefix
}

fn main() {
    let strings = vec!["april".to_string(), "apple".to_string(), "ape".to_string()];
    println!("{}", longest_common_prefix(strings));  // Output: "fl"
}
