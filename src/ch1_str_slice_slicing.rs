use anyhow::Result;

static SUBSTRATE: &str = "Substrate is a next-generation framework for blockchain innovation.";

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
    let mut words = String::new();

    std::io::stdin().read_line(&mut words)?;

    let words = Slices::slice(&words);

    for word in &words.slices {
        println!("{}", word);
    }

    Ok(())
}