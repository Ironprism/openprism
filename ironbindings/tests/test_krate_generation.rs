use ironbindings::{python::python_bindgen, rustdoc::doc_crate};
use quote::ToTokens;

#[test]
fn krate_generation() {
    stderrlog::new().verbosity(3).init().unwrap();
    let krate = doc_crate("tests/krate_generation").unwrap();
    std::fs::write("dbg.txt", format!("{:#2?}", krate)).unwrap();
    let res = python_bindgen(&krate).unwrap();
    std::fs::write("dbg_res.rs", res.to_token_stream().to_string()).unwrap();
}
