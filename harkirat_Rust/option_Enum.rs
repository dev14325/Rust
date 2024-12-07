// pub enum Option<T>{
// None,
// Some(T),
// }

// whenever you need to return null,return an option instead


fn find_first_a(s:String)->Option<i32>{
    for(index,value) in s.chars().enumerate(){
        if value== 'a' {
            return Some(index as i32)
        }
    }
    return None;

}

fn main(){
    let str = String::from("raman");
    let res = find_first_a(str);

    match res {
        Some(index)=>
        {
            println!("Index of 1st a is : {}",index);
        }
        None=>{
            println!("a not found");
        }
    }
}