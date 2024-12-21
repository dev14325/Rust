fn main(){

    // consuming adapter 
    let nums = vec![1,2,4];
    // let iter = nums.into_iter();


    // for value in iter {
    //     println!("{}",value);
    // }

    // let iter = nums.iter();
    // let sum :i32 = iter.sum();
    // println!("Sum is {}",sum); 

    // for val in iter {
    //     print!("{}",val);  // can't do this becuase iter move to sum function
    // }

    // iterrator adaptors

//     let iter = nums.iter();
//     let iter2 = iter.map(|x| x*2); // Rust "auto-dereferences" in closures for many methods like map, so you avoid dealing with &.
//     for val in iter2 {
//         println!("{}",val)
// ;    }

let iter = nums.iter();
let iter2 =  iter.filter(|&x| x%2==0);

for val in iter2{
    println!("{}",val);  // Destructure in filter: Use |&x| in filter to unpack the reference, or simply use |x| *x.
}


}