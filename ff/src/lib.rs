#![cfg_attr(not(feature = "std"), no_std)]
#![warn(unused, future_incompatible, nonstandard_style, rust_2018_idioms)]
#![allow(clippy::op_ref, clippy::suspicious_op_assign_impl)]
#![cfg_attr(not(use_asm), forbid(unsafe_code))]
#![cfg_attr(use_asm, feature(llvm_asm))]

#[macro_use]
extern crate ark_std;

#[macro_use]
extern crate derivative;

#[cfg_attr(test, macro_use)]
pub mod bytes;
pub use self::bytes::*;

#[macro_use]
pub mod biginteger;
pub use self::biginteger::*;

#[macro_use]
pub mod fields;
pub use self::fields::*;

// This is only used for testing.
#[cfg(test)]
mod test_field;

pub use ark_std::UniformRand;

mod rand;
#[allow(deprecated)]
pub use self::rand::test_rng;

mod to_field_vec;
pub use to_field_vec::ToConstraintField;

pub use num_traits::{One, Zero};

pub use ark_std::vec;

pub mod prelude {
    pub use crate::biginteger::BigInteger;

    pub use crate::fields::{Field, FpParameters, PrimeField, SquareRootField};

    pub use ark_std::UniformRand;

    pub use num_traits::{One, Zero};
}

fn error(msg: &'static str) -> ark_std::io::Error {
    ark_std::io::Error::new(ark_std::io::ErrorKind::Other, msg)
}
