#![no_main]
use rayon::iter::plumbing::Producer;
use arbitrary::Arbitrary;
use csr::prelude::*;
use csr::iter::CSRCoordinatesIter;
use libfuzzer_sys::fuzz_target;

#[derive(Arbitrary, Debug)]
struct FuzzCase {
    n_of_nodes: u8,
    data: Vec<((u32, u32), bool)>,
}

fuzz_target!(|data: FuzzCase| {
    let mut data = data;
    data.n_of_nodes = data.n_of_nodes.max(1);
    let n_of_edges = data.data.len();
    let mut edges = Vec::with_capacity(n_of_edges);
    for ((src, dst), _) in data.data.iter().take(n_of_edges) {
        edges.push((*src % data.n_of_nodes as u32, *dst % data.n_of_nodes as u32));
    }
    edges.sort();

    let graph: CSR<Vec<u32>, Vec<u32>> = CSRBuilder::default()
        .number_of_edges(n_of_edges)
        .number_of_nodes(data.n_of_nodes as usize)
        .sequential()
        .sorted()
        .build(edges.iter().copied());

    let mut csr_iter: CSRCoordinatesIter<'_, Vec<u32>, Vec<u32>> = csr::iter::CSRCoordinatesIter::from(&graph);
    let mut edges_iter = edges.iter();

    for direction in data.data.iter().map(|(_, dir)| *dir) {
        if direction {
            assert_eq!(edges_iter.next(), csr_iter.next().as_ref());
        } else {
            assert_eq!(edges_iter.next_back(), csr_iter.next_back().as_ref());
        }
    }

    for i in 0..n_of_edges {
        let csr_iter: CSRCoordinatesIter<'_, Vec<u32>, Vec<u32>> = csr::iter::CSRCoordinatesIter::from(&graph);

        let (left_csr, right_csr) = csr_iter.split_at(i);
        let (left_edges, right_edges) = edges.split_at(i);
        assert_eq!(
            left_csr.collect::<Vec<_>>(),
            left_edges,
            "left i: {}", i
        );        
        assert_eq!(
            right_csr.collect::<Vec<_>>(),
            right_edges,
            "right i: {}", i
        );
    }
});
