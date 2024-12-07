// struct Point<T> {
//     x:T,
//     y:T,
// }


// fn main(){
//     let i_Point:Point<i32> = Point{x:5 , y:10};
//     let f_Point:Point<f64> = Point{x:1.2 , y:2.5};
//     let s_Point:Point<&str> = Point{x:"Hello",y:"world"};
//     println!("Integer Points are : {},{}",i_Point.x,i_Point.y);
//     println!("Floating Points are : {},{}",f_Point.x,f_Point.y);

//     println!("String Points are : {},{}",s_Point.x,s_Point.y);

// }


struct option<A,B,C> {
    x:A,
    y:B,
    z:B,
    j:C,
}


fn main(){
    let op1 = option {x:12,y:"hi",z:"hello",j:1.9};
    println!("options are : {} , {} , {} , {}",op1.x,op1.y,op1.z,op1.j);

}
