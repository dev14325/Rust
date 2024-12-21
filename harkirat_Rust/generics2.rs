fn main(){
    let bigger = largest(1,2);
    let bigger_char = largest('a','b');
    println!("{}",bigger);
    println!("{}",bigger_char);
}

fn largest<T: PartialOrd>(a:T,b:T) ->T {
   if a>b{
    a
   }
   else {
    b
   }
}