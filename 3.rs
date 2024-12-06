// Given a string of words, implement a function that returns the shortest word in the string.
fn find_shortest_word(input : &str)-> &str{

    let mut shortest_word = input;

    for word in input.split_whitespace(){
        if word.len() < shortest_word.len(){
            shortest_word = word;

        }
    }

    shortest_word


}
fn main(){

    let s1 = "Hello this is example";
    let ans = find_shortest_word(s1);
    println!("Shortest word : {}",ans);
  



}