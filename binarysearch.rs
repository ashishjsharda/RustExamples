
fn main(){
   let arr= [1, 2, 3, 4, 5];
    let target = 7;
    let result = binary_search(&arr, target);
    println!("result: {:?}", result);

}

fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut size = arr.len();
    if size == 0 {
        return None;
    }
    let mut base = 0;
    while size > 1 {
        let half = size / 2;
        let mid = base + half;
        let value = arr[mid];
        if value == target {
            return Some(mid);
        } else if value < target {
            base = mid;
            size -= half;
        } else {
            size = half;
        }
    }
    if arr[base] == target {
        Some(base)
    } else {
        None
    }
}
