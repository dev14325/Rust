trait Summary {
    fn summarise(&self)-> String ;
}

struct User {
    name : String,
    age : u32,

}

impl Summary for User {
    fn summarise(&self) ->String {
        return format!("The name is {},and the age is 
    {}" , self.name , self.age);
    }

}

fn main(){
    let user = User {
        name : String::from("abc"),
        age : 22,
    };

    println!("{}",user.summarise());
}