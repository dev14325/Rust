fn main(){
    let ans;
    let str1 = String::from("small");
    {
        let str2 = String::from("larger");
        ans = longest(&str1,&str2);
    }
  
   
    println!("{}",ans);
}

fn longest<'a>(a:&'a str , b:&'a str)->&'a str {
    if a.len()>b.len(){
        return a;
    }
    else {
        return b;
    }
}