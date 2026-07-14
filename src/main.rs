use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();

    let user_input = &args[1];

    println!("You said: {}", user_input);
}
