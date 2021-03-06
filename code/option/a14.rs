
fn search(dict: &[(i32, i32)], key: i32) -> Option<i32> {

    for &(k, v) in dict {
        if k == key {
            return Some(v);
        }
    }
    
    None
}

fn main() {
    let a = [(1, 20), (3, 40), (5, 80)];
     
    let r1 = search(&a[..], 7);

    let r2 = r1.unwrap_or(-1);

    println!("{}", r2);
}
