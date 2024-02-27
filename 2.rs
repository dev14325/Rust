// Given a sorted array of integers, implement a function that returns the index 
// of the first occurrence of a given number.


fn main(){
    let arr = [1,4,5,5,5,6,7,9]; // 4
    let ele = 5; // in this we are finding the first occurrence of 5
   

   let mut idx = 0;
    for index in 0..8{

        if arr[index]==ele{
            idx = index;
            break;

        }
    }

    println!("index is {} ", idx);



}