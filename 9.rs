// Reverse a string in Rust

fn main() {
    let mut string = String::from("singh");

    let mut i = 0;
    let mut j = string.len() - 1;

    while j > i {
        let temp = string.chars().nth(i).unwrap();
        string.replace_range(i..=i, &string.chars().nth(j).unwrap().to_string());
        string.replace_range(j..=j, &temp.to_string());
        i += 1;
        j -= 1;
    }

    println!("After reversing the string: {}", string);
}
