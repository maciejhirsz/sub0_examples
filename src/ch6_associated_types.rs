//!
//! Find this code at:
//! https://github.com/maciejhirsz/sub0_examples
//!
// std::std::FromStr;
// std::num::ParseIntError;

trait DoWork {
    type Work;
    type Error;

    fn do_work(&self) -> Result<Self::Work, Self::Error>;
}

struct Add5<'a>(&'a str);

pub fn run() -> anyhow::Result<()> {
    let mut input = String::new();

    std::io::stdin().read_line(&mut input)?;

    let life = Add5(&input).do_work()?;

    println!("{}", life);

    Ok(())
}