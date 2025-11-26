#![doc(html_playground_url = "https://play.rust-lang.org/")]
//! OrdoFP Laws
//!
//! This library contains laws that can be used to test the implementation of
//! algebras declared in OrdoFP

extern crate ordofp;
extern crate quickcheck;

pub mod monoid_laws;
pub mod semigroup_laws;
pub mod wrapper;
