//!
//! Find this code at:
//! https://github.com/maciejhirsz/sub0_examples
//!
use anyhow::Result;

struct Slices<'a> {
    slices: Vec<&'a str>,
}

impl<'a> Slices<'a> {
    fn slice(text: &'a str) -> Self {
        Slices {
            slices: text.split_whitespace().collect(),
        }
    }

    fn add(&mut self, slice: &'a str) {
        self.slices.push(slice);
    }
}

pub fn run() -> Result<()> {
    let mut input = String::new();

    std::io::stdin().read_line(&mut input)?;

    let words = Slices::slice(&input);

    for word in &words.slices {
        println!("{}", word);
    }

    Ok(())
}