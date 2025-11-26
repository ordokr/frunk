// Makes sure that the hlist macros in ordofp_core are reexported by ordofp
use ordofp::{hlist, hlist_pat, HList};

#[test]
fn use_ordofp_macros() {
    let h1 = hlist![1i32, 2u32];
    let h2 = hlist!["cool", ...h1];
    let hlist_pat![a, ...bs]: HList![&'static str, i32, ...HList![u32]] = h2;
    assert_eq!(a, "cool");
    assert_eq!(bs, h1);
}
