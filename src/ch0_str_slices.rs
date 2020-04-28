//!
//! Find this code at:
//! https://github.com/maciejhirsz/sub0_examples
//!
fn sub0_ref_str() -> &str {
    let string = String::new("sub0");

    &string
}

fn run() {
    println!("Hello, {}!", sub0_ref_str());
}