// Implement a function that returns the kth smallest element in a given array.


// {7, 10, 4, 3, 20, 15}, K = 3  Output: 7


fn main() {
    let mut arr = vec![7, 10, 4, 3, 20, 15];
    let k = 3; 

    arr.sort(); 

    if k <= arr.len() {
        println!("Answer is {}", arr[k - 1]); 
    } else {
        println!("Error: k is out of range");
    }
}
