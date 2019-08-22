fn main() {
    let mut s = String::from("Hello, World");
    let index = get_first_word(&s);
    println!("{}", index);
    s.clear();
    println!("{}", &s[0..2]);
    println!("{}", index);
}

fn get_first_word(str: &String) -> &str {
    let str_bytes = str.as_bytes();
    for (index, &string_char) in str_bytes.iter().enumerate() {
        if string_char == b' ' {
            return &str[0..index];
        }
    }
    &str[0..]
}
