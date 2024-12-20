fn main(){
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(8);
    // println!("{:?}",vec);

    


fn temp(vec : Vec<i32>){
    let mut new_vec = Vec::new();
    for val in vec {
        if val%2==0 {
            new_vec.push(val);
        }
    }
    
println!("{:?}",new_vec);
}


temp(vec);

}