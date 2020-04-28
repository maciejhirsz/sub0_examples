


fn sub0_ref_str() -> &str {
    let string = String::new("sub0");

    &string
}

fn run() {
    println!("Hello, {}!", sub0_ref_str());
}