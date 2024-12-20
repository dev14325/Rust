use std::collections::HashMap;


fn temp(vec:Vec<(String,i32)>)-> HashMap<String,i32> {
    let mut hm = HashMap::new();
    for(key,val) in vec {
        hm.insert(key,val);
    }
    return hm;
}

fn main(){
    let input_user = vec![(String::from("abc"),32),(String::from("def"),22)];
    let hm = temp(input_user);
    print!("{:?}",hm);
}