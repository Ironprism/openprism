use anyhow::Result;

/// Given the list of objects, generate the __getattr__ method
/// that will suggest the closest match to the user
pub fn gen_getattr(names: &[String], method: bool) -> Result<syn::ItemFn> {
    let attr = if method {
        quote::quote!()
    } else {
        quote::quote!(#[pyfunction])
    };
    
    if names.is_empty() {
        return Ok(syn::parse_quote!(
            #attr
            pub fn __getattr__(&self, name: &str) -> PyResult<()> {
                Err(PyAttributeError::new_err(format!(
                    "`{{}}` does not exists becasue this object does not have any attributes.",
                    name
                )))
            }
        ));
    }
    // generate the weights for the tfidf
    let crate::tfidf::TFIDF{
        unique_terms_list,
        tfidf,
    } = crate::tfidf::tfidf_gen(names)?;
    // convert the tfidf result to a list of token streams so we can use it in
    // the quote
    let mut tfidfs_rows = Vec::new();
    for row in tfidf.iter() {
        let vals = row.iter().map(|(name, score)| quote::quote!((#name, #score)));
        tfidfs_rows.push(quote::quote!(&[ #(#vals),* ]));
    }

    Ok(syn::parse_quote!(
        #attr
        pub fn __getattr__(&self, name: &str) -> PyResult<()> {
            const NAMES: &[&str] = &[#(#names),*];
            const TERMS: &[&str] = &[#(#unique_terms_list),*];
            const TFIDF_FREQUENCIES: &[&[(&str, f64)]] = &[#(#tfidfs_rows),*];

            Err(PyAttributeError::new_err(format!(
                "`{{}}` does not exists, did you mean one of the following?\n\n{{}}",
                name,
                tfidf_splitter(name, TERMS, NAMES, TFIDF_FREQUENCIES)
                    .map(|method| {{
                        format!("* `{}`",  method )
                    }})
                    .take(10)
                    .collect::<Vec<String>>()
                    .join("\n"),
            )))
        }
    ))
}