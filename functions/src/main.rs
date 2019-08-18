fn main() {
    println!("Hello, world!");
    let data = another_function(10, 20);
    println!("data value:{}", data);
}

fn another_function(x: i32, y: i32)  ->i32{
    let z = {
        let x = 30;
        x + y
    };

    println!("The value of x is:{}", x);
    println!("The value of y is:{}", y);
    println!("The value of z is:{}", z);
    x + y
}