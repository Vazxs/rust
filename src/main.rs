fn main() {

    let number:i32= 6;

    match number {
        1 => println!(" value is one."),
        2 => println!(" value is two."),
        3..=8 => println!(" value is b/w 3 to 8."),
        _=> println!("default value."),
    }
}
