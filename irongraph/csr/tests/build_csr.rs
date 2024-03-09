use csr::prelude::*;
use epserde::{deser::Deserialize, ser::Serialize};
use graph::*;
use irontraits::*;

#[test]
fn test_build_csr() {
    let edges: Vec<(usize, usize)> = vec![(0, 1), (0, 2), (1, 2), (1, 3), (2, 3), (3, 4)];
    let number_of_edges = edges.len();

    let csr: CSR<Vec<usize>, Vec<usize>> = CSRBuilder::default()
        .number_of_edges(number_of_edges)
        .number_of_nodes(5)
        .sorted()
        .build(edges);

    assert_eq!(csr.number_of_nodes(), 5);
    assert_eq!(csr.number_of_edges(), number_of_edges);
    assert_eq!(
        csr.successors(0).into_iter().collect::<Vec<_>>(),
        vec![1, 2]
    );
    assert_eq!(
        csr.successors(1).into_iter().collect::<Vec<_>>(),
        vec![2, 3]
    );
    assert_eq!(csr.successors(2).into_iter().collect::<Vec<_>>(), vec![3]);
    assert_eq!(csr.successors(3).into_iter().collect::<Vec<_>>(), vec![4]);
    assert_eq!(csr.successors(4).into_iter().collect::<Vec<_>>(), vec![]);

    let path = std::env::temp_dir().join("csr.csr");
    csr.store(&path).unwrap();

    let csr2 = CSR::<Vec<usize>, Vec<usize>>::mmap(&path, Default::default()).unwrap();

    assert_eq!(csr2.number_of_nodes(), 5);
    assert_eq!(csr2.number_of_edges(), number_of_edges);
    assert_eq!(
        csr2.successors(0).into_iter().collect::<Vec<_>>(),
        vec![1, 2]
    );
    assert_eq!(
        csr2.successors(1).into_iter().collect::<Vec<_>>(),
        vec![2, 3]
    );
    assert_eq!(csr2.successors(2).into_iter().collect::<Vec<_>>(), vec![3]);
    assert_eq!(csr2.successors(3).into_iter().collect::<Vec<_>>(), vec![4]);
    assert_eq!(csr2.successors(4).into_iter().collect::<Vec<_>>(), vec![]);
}
