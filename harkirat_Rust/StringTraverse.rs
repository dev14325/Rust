 fn main(){
    let s : String = String :: from("My name is Dev");
    let ans = get_first_word(s);
    print!("First word of the sentence is : {}",ans);
 }


fn get_first_word(sentence : String)->String{
    let mut ans : String = String::from(" ");

    for c in sentence.chars(){
        ans.push(c);

        if c ==' '{
            break;
        }

    }

    return ans;

}

 