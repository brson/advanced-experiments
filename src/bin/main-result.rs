#[macro_use] extern crate failure;

use failure::{Error, ResultExt};

fn main() -> Result<(), Error> {
    run_foo().context("foo failed")?;
    Ok(())
}

fn run_foo() -> Result<(), Error> {
    run_bar().context("bar failed")?;
    Ok(())
}

fn run_bar() -> Result<(), Error> {
    bail!("something really bad happened")
}
