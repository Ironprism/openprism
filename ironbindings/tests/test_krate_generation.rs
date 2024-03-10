use ironbindings::{
    parse::parse_crate, python::python_bindgen, rustdoc::doc_crate
};
use quote::ToTokens;

#[test]
fn krate_generation() {
    let krate = doc_crate("tests/krate_generation").unwrap();
    let module = parse_crate(&krate);
    std::fs::write("dbg.txt", format!("{:#2?}", krate)).unwrap();
    let res = python_bindgen(&module).unwrap();
    std::fs::write("dbg_res.rs", res.to_token_stream().to_string()).unwrap();
}