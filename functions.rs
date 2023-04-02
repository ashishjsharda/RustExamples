
fn main(){
  hello();
  println!("2 + 2 = {}", add(2, 2));

}

fn hello(){
  println!("Hello, world!");
}

fn add(left: usize, right: usize) -> usize {
    left + right
}
