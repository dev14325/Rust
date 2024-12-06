fn main(){


// 1st way 
//    let s1:String = String::from("hello");
//    let s2 = s1; // get rid of dangling pointer & double free error
//    print!("value of s2 is : {}",s2);


// 2nd way


// let s1:String = String::from("hello");
// takes_ownership(s1);
// }

// fn takes_ownership(s2:String){
//     print!("{}",s2); // s2 owns the data
// }


// Returning of Rihana to her 1st boyfriend... make it possible


let mut s1:String = String::from("hello");
s1 = takes_ownership2(s1);
println!("s1 is - {} ",s1);

}


fn takes_ownership2(s2:String)->String{
   return s2;
    // println!("{}",s2);
}