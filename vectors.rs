
fn main() {
   let mut numbers = vec![1, 2, 3, 4, 5];
    let v1_iter = numbers.iter();
    for val in v1_iter {
        println!("Got: {}", val);
    }
}
