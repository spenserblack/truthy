use truthy::Truthy;
use std::fmt::Display;

fn main() {
    println!("ok if truthy:");
    println!("{:?} => {:?}", 1.5, ok_if_truthy(1.5));
    println!("{:?} => {:?}", 0i8, ok_if_truthy(0i8));
    println!("{:?} => {:?}", "hello", ok_if_truthy("hello"));
    println!("{:?} => {:?}", "", ok_if_truthy(""));
}

fn ok_if_truthy<T: Truthy + Display>(value: T) -> Result<String, String> {
    if value.truthy() {
        Ok(format!("{} is valid!", value))
    } else {
        Err(format!("{} is not valid :(", value))
    }
}
