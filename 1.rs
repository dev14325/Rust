// palindrome or not // cat , naman 

fn main(){
   
let mut s1 = String::new();

 println!("Enter your string");
   

   let _ = std::io::stdin().read_line(&mut s1).unwrap();
   // input = dev;

   let trimmed_s1 = s1.trim();

   
  if trimmed_s1  == trimmed_s1 .chars().rev().collect::<String>() {
    println!("Given String is palindrome");
  }
  else{
    println!("Given String is not palindrome");
  }



  
}