#![allow(warnings)]

extern crate failure;

mod a {

    pub struct A1;
    pub struct A2;
    pub struct A3;
    pub struct A4;

    pub mod b {
        pub struct B1;
        pub struct B2;
        pub struct B3;
        pub struct B4;
        pub struct B5;
    }

    #[cfg(feature = "rust-2015")]
    use C1;
    #[cfg(feature = "rust-2015")]
    use ::C2;
    use self::super::C3;
    use super::C4;
    use crate::C5;

    use failure::Fail;
    use ::failure::Error;
    use self::super::failure::err_msg;
    use crate::failure::ResultExt;

    #[cfg(feature = "rust-2015")]
    use a::b::B1;
    #[cfg(feature = "rust-2015")]
    use ::a::b::B2;
    use self::b::B3;
    use super::a::b::B4;
    use crate::a::b::B5;
}

struct C1;
struct C2;
struct C3;
struct C4;
struct C5;

use a::A1; // "a" is in scope
#[cfg(feature = "rust-2015")]
use ::a::A2;
use self::a::A3;
use crate::a::A4;

use failure::Fail;
use ::failure::Error;
use self::failure::err_msg;
use crate::failure::ResultExt;

fn main() { }
