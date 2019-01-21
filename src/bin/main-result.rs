use failure::{Error, err_msg, ResultExt};

fn main() -> Result<(), Error> {
    run_foo().context("foo failed")?;
    Ok(())
}

fn run_foo() -> Result<(), Error> {
    run_bar().context("bar failed")?;
    Ok(())
}

fn run_bar() -> Result<(), Error> {
    Err(err_msg("something really bad happened"))
}
