fn main() {
    let mut data: String = String::from("hello World");
    let data2: String;

    let a = &data;
    let b = &data;
    let c = &mut data;
    println!("{}{}", c, data);
    //data2 = data;
    // Not allowed because data moved to data2
    // print_string(data);

    // print_string(data);
    // Not allowed because data moved to print_string scope
    // data2 = data;

    print_string(&mut data);
    // data2 = data;

    // let data3 = String::from("HI");
    return_data(data);

    // Not allowed
    //println!("{}",data3);

    let mut data4 = String::from("hi");
    data4 = return_data(data4);
    println!("{}", data4)
}

fn print_string(value: &mut String) {
    println!("{}", value);
}

fn return_data(data: String) -> String {
    data
}