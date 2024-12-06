fn main(){
    let s:String = String::from("Hi There");
    // println!("{}",s);
    let char1: Option<char> = s.chars().nth(1); 

    // match char1 {
    //     Some(c: char) => println!("{}",c),
    //     None => println!("No charcater at 0th index"),
    // }

    // we can unwrap a options of character

    print!("char1 : {} ",char1.unwrap());  // run time exception may occur when index is exceeded 


}