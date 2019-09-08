fn main() {
    let mut vec = Vec::new();
    vec.push(2);
    let vec1 = vec![1, 2, 3];
    println!("value {}", &vec1[1]);
    let data = vec1.get(8);
    println!("{}", data.is_some());
}
