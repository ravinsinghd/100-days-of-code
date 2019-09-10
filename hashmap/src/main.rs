use std::collections::HashMap;
fn main() {
    let mut map = HashMap::new();
    let data = String::from("Ravin");
    map.insert("test", &data);
    println!("{}", data);
    printValue(data);
    // map.iter();
}
fn printValue(data: String) {
    println!("{}", data);
}
