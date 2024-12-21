fn main(){
    // let mut vec = Vec::new();
    // vec.push(1);
    // vec.push(2);
    // vec.push(3);
    // vec.push(8);
   

//    let iter = vec.iter();
//    for value in iter {
//     print!("{}",value);
//    }
    


// let nums = vec![1,2,3];
// let  iter = nums.iter();

// while let Some(val) = iter.next() {
//     print!("{}",val);
// }

// Mutable iterator
let mut nums  = vec![1,2,3];
let iter = nums.iter_mut();

for value in  iter {
    *value = *value + 1;
}

println!("{:?}",nums);

}