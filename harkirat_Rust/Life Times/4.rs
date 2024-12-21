// struct with lifetime

struct User<'a> {
    name : &'a str,
}

fn main(){
    let user;
    let name = String::from("Singh");
    user = User{name : &name};
    println!("{}",user.name);
}