#![feature(test)]
extern crate test;
use csr::prelude::*;
use rand::{Rng, SeedableRng};
use test::{black_box, Bencher};

const NODES: u32 = 100_000;
const EDGES: usize = 1_000_000;
const SEED: [u8; 32] = [
    0xf9, 0xe4, 0x62, 0xd3, 0x28, 0xe8, 0x41, 0x42, 0x43, 0xe3, 0x04, 0x55, 0xcc, 0xe6, 0x70, 0x37,
    0x78, 0x42, 0x33, 0xb8, 0xe7, 0x40, 0x9b, 0x54, 0x50, 0xb0, 0x51, 0xf2, 0xd7, 0x96, 0xf1, 0xf3,
];

fn random_graph() -> CSR<Vec<u32>, Vec<u32>> {
    let mut prng = rand::rngs::SmallRng::from_seed(SEED);

    let mut edges = (0..EDGES)
        .map(|_| {
            let src = prng.gen::<u32>() % NODES;
            let dst = prng.gen::<u32>() % NODES;
            (src, dst)
        })
        .collect::<Vec<_>>();
    edges.sort();

    CSRBuilder::default()
        .number_of_edges(edges.len())
        .number_of_nodes(NODES as usize)
        .sequential()
        .sorted()
        .build(edges)
}

#[bench]
fn csr_iter(b: &mut Bencher) {
    let graph = random_graph();
    b.iter(|| {
        let mut sum = 0;
        let iter: csr::iter::CSRCoordinatesIter<_, _> = (&graph).into();
        for (src, dst) in iter {
            sum += src + dst;
        }
        black_box(sum);
    });
}

#[bench]
fn generic_iter(b: &mut Bencher) {
    let graph = random_graph();
    b.iter(|| {
        let mut sum = 0;
        let iter: graph::iter::CoordinatesIter<_> = (&graph).into();
        for (src, dst) in iter {
            sum += src + dst;
        }
        black_box(sum);
    });
}
