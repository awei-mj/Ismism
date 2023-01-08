use ismism;
use std::{env, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args();
    ismism::process(args)?;

    Ok(())
}
