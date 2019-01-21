use failure::{Error, err_msg, ResultExt};
use std::process;

fn main() {
    if let Err(e) = run() {
        println!("program failed: {}", e);
        let num_causes = e.iter_causes().count();
        for (i, cause) in e.iter_causes().enumerate() {
            if num_causes != i + 1 {
                println!("  caused by: {}", cause);
            } else {
                println!("  root cause: {}", cause);
            }
        }
        process::exit(1);
    }
}

fn run() -> Result<(), Error> {
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
