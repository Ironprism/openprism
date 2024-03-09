#![deny(unconditional_recursion)]
use core::borrow::Borrow;
use irontraits::PositiveInteger;

pub mod iter;

pub trait Graph {
    type Node: PositiveInteger;

    fn number_of_nodes(&self) -> usize;
    fn number_of_edges(&self) -> usize;

    type Nodes: IntoIterator<Item = Self::Node>;
    fn nodes(&self) -> Self::Nodes;

    fn undirected(&self) -> bool {
        !self.directed()
    }
    
    fn directed(&self) -> bool;
}

pub trait Successors: Graph {
    type Successors<'a>: IntoIterator<Item = Self::Node> + 'a
    where
        Self: 'a;

    // panic if node doesn't exist
    fn successors<N: Borrow<Self::Node>>(&self, node: N) -> Self::Successors<'_>;
    // returns false if src or dst don't exist
    fn has_successor<S: Borrow<Self::Node>, D: Borrow<Self::Node>>(&self, src: S, dst: D) -> bool;
}
