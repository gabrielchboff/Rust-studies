use std::io;

fn main() {
    let mut age = String::new();
    println!("Type your age: ");
    io::stdin()
        .read_line(&mut age)
        .expect("Failed to read the line");
    println!("This is your age: {age}");
}
