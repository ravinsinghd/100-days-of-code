fn main() {
    let data: String = String::from("hello World");
    let data2: String;

    // data2 = data;
    // Not allowed because data moved to data2
    // print_string(data);

    // print_string(data);
    // Not allowed because data moved to print_string scope
    // data2 = data;

    print_string(data);

    let data3 = String::from("HI");
    return_data(data3);

    // Not allowed
    //println!("{}",data3);

    let mut data4 = String::from("hi");
    data4 = return_data(data4);
    println!("{}",data4)
}

fn print_string(value: String) {
    println!("{}", value);
}

fn return_data(data: String) -> String {
    data
}