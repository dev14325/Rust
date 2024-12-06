fn main(){
    // let s1: String = String::from("hello");
    // let s2: &String = &s1;

    // println!("{}",s1);
    // print!("{}",s2);

let mut s1:String = String::from("hello");
update_str(&mut s1);
println!("{}",s1);

}

fn update_str(s: &mut String){
    s.push_str(" world");
    
}


