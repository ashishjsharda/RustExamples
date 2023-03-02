use std::collections::HashMap;

fn main(){
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Black"), 20);
    println!("{:?}", scores);
    let count = scores.entry(String::from("Blue")).or_insert(0);
    println!("{:?}", count);
}
