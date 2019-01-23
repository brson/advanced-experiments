#![allow(warnings)]

mod a {
    pub struct A1;
    pub struct A2;
    pub struct A3;
    pub struct A4;
}

use a::A1; // "a" is in scope
#[cfg(feature = "rust-2015")]
use ::a::A2; // :: is only for crates in 2018
use self::a::A3;
use crate::a::A4;

fn f1() {
    let _ = a::A1;
    #[cfg(feature = "rust-2015")]
    let _ = ::a::A2; // incorrect error message? #57849
    let _ = self::a::A3;
    let _ = crate::a::A4;
}

extern crate failure;

use failure::Error;
use ::failure::Backtrace;
use self::failure::Causes;
use crate::failure::Compat;

fn f3() {
    let _ = failure::Error;
    let _ = ::failure::Backtrace;
    let _ = self::failure::Causes;
    let _ = crate::failure::Compat;
}

struct B1;
struct B2;
struct B3;
struct B4;
struct B5;

mod c {

    #[cfg(feature = "rust-2015")]
    use B1; // root in 2015, out of scope in 2018
    #[cfg(feature = "rust-2015")]
    use ::B2; // root in 2015, not a crate in 2018
    use self::super::B3;
    use super::B4;
    use crate::B5;

    fn f1() {
        #[cfg(feature = "rust-2015")]
        let _ = B1;
        #[cfg(feature = "rust-2015")]
        let _ = ::B2;
        let _ = self::super::B3;
        let _ = super::B4;
        let _ = crate::B5;
    }

    pub mod d {
        pub struct D1;
        pub struct D2;
        pub struct D3;
        pub struct D4;
        pub struct D5;
    }

    #[cfg(feature = "rust-2015")]
    use c::d::D1;
    #[cfg(feature = "rust-2015")]
    use ::c::d::D2;
    use self::d::D3;
    use super::c::d::D4;
    use crate::c::d::D5;

    fn f2() {
        // works in neither edition (thus inconsistent in 2015)
        // let _ = c::d::D1;
        #[cfg(feature = "rust-2015")]
        let _ = ::c::d::D2;
        let _ = self::d::D3;
        let _ = super::c::d::D4;
        let _ = crate::c::d::D5;
    }

    use failure::Error;
    use ::failure::Backtrace;
    use self::super::failure::Causes;
    use crate::failure::Compat;

    fn f2() {
    }
}

use c::d;
use d::D1; // "b" is in scope

// This just shows that resolution isn't in lexical (?) order
use f::F1;
use e::f;
mod e { pub mod f { pub struct F1; } }

// resolution of name conflicts
mod g {
    mod failure {
        pub struct Error;
        pub struct MyError;
    }

    // root takes precedence in 2015; ambiguous in 2018
    #[cfg(feature = "rust-2015")]
    use failure::Error;
    // use failure::MyError;
    use self::failure::MyError;
}


fn main() { }
