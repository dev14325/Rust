fn main(){
    let name = String::from("This is ex");
    let ans = first_word(&name);
    println!("ans is {}",ans);
}


fn first_word(str : &String) ->&str{
    let mut i =0;
    for j in str.chars(){
        if j == ' '{
            break;
        }   
        i = i+1;
    }

    return &str[0..i];
}