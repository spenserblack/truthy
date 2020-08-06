use truthy::Truthy;

fn main() {
    println!(r#"true.truthy_and("Hi!") => {:?}"#, true.truthy_and("Hi!"));
    println!(r#"false.truthy_and("Hi!") => {:?}"#, false.truthy_and("Hi!"));
    println!(r#"true.truthy_or("Hi!") => {:?}"#, true.truthy_or("Hi!"));
    println!(r#"false.truthy_or("Hi!") => {:?}"#, false.truthy_or("Hi!"));
}
