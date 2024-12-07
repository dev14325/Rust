use std::fs;

// enum Result<A,B> {
//     Ok(A),
//     Err(B),// B is generally a struct
// } // this is already implemented in rust

fn main(){
    let res = fs::read_to_string("example.txt");
    // res.unwarp() // thread might get crashed
    // res will either be Ok or Err
    match res { // pattern matching
        Ok(content)=>{
            println!("File content : {}",content);

        },
        Err(err)=>{
            println!("Error : {}",err);
        }
    }

}