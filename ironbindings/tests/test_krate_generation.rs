use ironbindings::{
    parse::parse_crate,
    rustdoc::doc_crate,
};

#[test]
fn krate_generation() {
    let krate = doc_crate("tests/krate_generation").unwrap();
    std::fs::write("dbg.txt", format!("{:#2?}", parse_crate(&krate))).unwrap();
}