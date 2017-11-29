fn main() {
    let z = vec![1,2,3];
    let  t = |x, y| x + y + z[0];
    println!("{}", t(1, 2));

}
