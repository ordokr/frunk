#![no_main]
use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use ordofp::{from_generic, into_generic, Generic};

#[derive(Generic, Debug, PartialEq, Arbitrary, Clone)]
struct FuzzStruct {
    a: i32,
    b: u8,
    c: bool,
    d: String,
}

fuzz_target!(|data: FuzzStruct| {
    // Round trip test
    let h = into_generic(data.clone());
    let s2: FuzzStruct = from_generic(h);
    assert_eq!(data, s2);
});
