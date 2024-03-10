use std::collections::BTreeMap;
use std::hash::Hash;
use itertools::Itertools;
use crate::utils::Counter;
use anyhow::{ensure, Result};

pub fn tfidf_splitter<S: AsRef<str>>(method_name: S) -> Vec<String> {
    method_name.as_ref()
        .split("_")
        .filter(|x| !x.is_empty())
        .map(|x| x.to_lowercase())
        .collect()
}

/// Return the tfidf_splitter function as a TokenStream so it can be used in a proc macro.
pub fn get_tfidf_splitter() -> syn::ItemFn {
    syn::parse_quote! {
        fn tfidf_splitter<S: AsRef<str>>(method_name: S) -> Vec<String> {
            method_name.as_ref()
                .split("_")
                .filter(|x| !x.is_empty())
                .map(|x| x.to_lowercase())
                .collect()
        }
    }
}

/// Return the tfidf_matcher function as a TokenStream so it can be used in a proc macro.
pub fn get_tfidf_matcher() -> syn::ItemFn {
    syn::parse_quote! {
        fn tfidf_splitter<'a, S: AsRef<str>>(
            name: S,
            terms: &[&str],
            methods_names: &[&'a str],
            tfidf_frequencies: &[&[(&str, f64)]],
        ) -> impl Iterator<Item = &'a str>{
            // compute the similarities between all the terms and tokens
            let tokens_expanded = tfidf_splitter(&name).iter()
                .map(|token| {
                    let mut similarities = terms
                        .iter()
                        .map(move |term| (*term, stdsim::jaro_winkler(token, term) as f64))
                        .collect::<Vec<(&str, f64)>>();

                    similarities.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());

                    similarities.into_iter().take(1)
                })
                .flatten()
                .collect::<Vec<(&str, f64)>>();

            // Compute the weighted ranking of each method ("document")
            // where the conribution of each term is weighted by it's similarity
            // with the query tokens
            let mut doc_scores = tfidf_frequencies
                .par_iter()
                .enumerate()
                // for each document
                .map(|(id, frequencies_doc)| {
                    let score: f64 = frequencies_doc.iter()
                        .map(|(term, weight)| {
                            match tokens_expanded.iter().find(|(token, _)| token == term) {
                                Some((_, similarity)) => similarity * weight,
                                None => 0.0,
                            }
                        }).sum();
                    (
                        id,
                        score * stdsim::jaro_winkler(&name, methods_names[id]),
                    )
                })
                .collect::<Vec<(usize, f64)>>();

            // sort the scores in a decreasing order
            doc_scores.sort_by(|(_, d1), (_, d2)| d2.partial_cmp(d1).unwrap());

            doc_scores.into_iter()
                .map(|(method_id, _)| methods_names[method_id])
        }
    }
}

pub struct TFIDF {
    pub unique_terms_list: Vec<String>,
    pub tfidf: Vec<Vec<(String, f32)>>,
}

/// Pre-compute the TD-IDF weight for each term of each binding.
/// Then write the compute weights in a file at the given path.
pub fn tfidf_gen<V, S>(method_names: V) -> Result<TFIDF> 
where
    V: AsRef<[S]>,
    S: AsRef<str>,
{
    // split the method names into sub-words
    let documents = method_names.as_ref()
        .iter()
        .map(|x| tfidf_splitter(x))
        .collect::<Vec<Vec<String>>>();

    // compute the TD-IDF weights
    let tfidf = okapi_bm25_tfidf_from_documents(&documents, 1.5, 0.75)?;

    let unique_terms_list = documents
        .iter()
        .flat_map(|document| document.into_iter())
        .cloned()
        .unique()
        .collect();

    Ok(TFIDF {
        unique_terms_list,
        tfidf: tfidf.into_iter()
            .map(|vals| {
                vals.into_iter()
                    .map(|(k, v)| (k.to_string(), v))
                    .collect::<Vec<(String, f32)>>()
            })
            .collect::<Vec<Vec<(String, f32)>>>(),
    })
}

/// Return vector of BTreeMaps containing the non-zero frequencies.
///
/// # Arguments
/// * `documents`: &[Vec<T>] - The documents to be processed.
/// * `k1`: f32 - The default parameter for k1, tipically between 1.2 and 2.0.
/// * `b`: f32 - The default parameter for b, tipically equal to 0.75.
pub fn okapi_bm25_tfidf_from_documents<T, D, V>(
    documents: D,
    k1: f32,
    b: f32,
) -> Result<Vec<BTreeMap<T, f32>>>
where
    D: AsRef<[V]>,
    V: AsRef<[T]>,
    T: Eq + Hash + Send + Sync + Clone + Eq + Ord,
{
    let documents = documents.as_ref();
    ensure!(!documents.is_empty(), "The given documents set is empty!");

    let number_of_documents = documents.len();
    // We start to iterate over the documents list and create the vocabulary.
    let vocabulary: BTreeMap<&T, usize> = documents
        .iter()
        .flat_map(|document| document.as_ref().iter())
        .unique()
        .enumerate()
        .map(|(element_id, element)| (element, element_id))
        .collect();
    // We start to compute, for each word, the number of documents that contain this word.
    let mut unique_document_occurrencies_per_word: Vec<usize> = vec![0; vocabulary.len()];
    let total_documents_length: usize = documents
        .iter()
        .map(|document| {
            document.as_ref().iter().unique().for_each(|element| {
                let idx = *vocabulary.get(element).unwrap();
                unique_document_occurrencies_per_word[idx] += 1;
            });
            document.as_ref().len()
        })
        .sum();
    // Computing average document size
    let average_document_len = total_documents_length as f32 / number_of_documents as f32;
    // Computing TFIDF of provided words and documents
    Ok(documents
        .iter()
        .map(|document| {
            let document_len = document.as_ref().len() as f32;

            let mut counter = Counter::new();
            document.as_ref()
                .iter()
                .for_each(|word| {
                    counter.insert(word);
                });
                
            counter.into_iter()
                .map(|(word_name, current_document_word_count)| {
                    // Surely the word is, by definition in the vocabulary.
                    let word_id = *vocabulary.get(&word_name).unwrap();
                    let word_frequency = current_document_word_count.clone() as f32 / document_len;
                    let unique_document_occurrencies =
                        unique_document_occurrencies_per_word[word_id] as f32;
                    // Computing the inverse document frequency
                    let inverse_document_frequency =
                        ((number_of_documents as f32 - unique_document_occurrencies + 0.5)
                            / (unique_document_occurrencies + 0.5))
                            .ln_1p();
                    // Computing the adjusted word frequency
                    let adjusted_word_frequency = (word_frequency * (k1 + 1.0))
                        / (word_frequency
                            + k1 * (1.0 - b + b * document_len / average_document_len));
                    (
                        word_name.clone(),
                        inverse_document_frequency * adjusted_word_frequency,
                    )
                })
                .collect::<BTreeMap<T, f32>>()
        })
        .collect::<Vec<BTreeMap<T, f32>>>())
}
