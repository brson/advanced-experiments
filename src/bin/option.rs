fn main() {
    println!("{:?}", option_success());
    println!("{:?}", option_failure());
}

fn option_success() -> Option<i32> {
    let value = get_thing_some()?;
    Some(value + 1)
}

fn option_failure() -> Option<i32> {
    let value = get_thing_none()?;
    Some(value + 1)
}

fn get_thing_some() -> Option<i32> { Some(1) }
fn get_thing_none() -> Option<i32> { None }
