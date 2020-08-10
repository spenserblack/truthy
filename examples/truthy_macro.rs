use truthy::{Truthy, truthy};

fn main() {
    let x = 1; // truthy
    let y = 0; // falsy
    let z = ""; // falsy
    println!("let x = 1; // truthy");
    println!("let y = 0; // falsy");
    println!(r#"let z = ""; // falsy"#);

    println!("truthy!(x && (y || !z)) => {:?}", truthy!(x && (y || !z)));
    println!("truthy!(x && (!y || z)) => {:?}", truthy!(x && (!y || z)));
    println!("truthy!((x && !y) || z) => {:?}", truthy!((x && !y) || z));
    println!("truthy!(!(x && y) || z) => {:?}", truthy!(!(x && y) || z));
    println!("truthy!(((!(!x) && !!!y) || z)) => {:?}", truthy!(((!(!x) && !!!y) || z)));
}
