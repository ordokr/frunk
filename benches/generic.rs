#![feature(test)]

#[macro_use]
extern crate ordofp;
extern crate ordofp_core;
extern crate test;

use ordofp::generic::*;
use test::Bencher;

#[derive(Generic)]
struct NewUser<'a> {
    first_name: &'a str,
    last_name: &'a str,
    age: usize,
}

#[derive(Generic)]
struct SavedUser<'a> {
    first_name: &'a str,
    last_name: &'a str,
    age: usize,
}

#[bench]
fn generic_conversion(b: &mut Bencher) {
    b.iter(|| {
        let n_u = NewUser {
            first_name: "Joe",
            last_name: "Schmoe",
            age: 30,
        };
        SavedUser::convert_from(n_u)
    })
}
