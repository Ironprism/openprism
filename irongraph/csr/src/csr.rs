use core::borrow::Borrow;
use epserde::Epserde;
use graph::{Graph, Successors};
use ironstructs::{custom_iters::copied::Copied, ranger::Ranger};
use irontraits::{PositiveInteger, SequenceLen, SequenceRandomAccess, To};

#[derive(Epserde, Debug, Clone)]
pub struct CSR<Destinations, Offsets> {
    pub(crate) destinations: Destinations,
    pub(crate) offsets: Offsets,
}

impl<Destinations, Offsets> CSR<Destinations, Offsets> {
    #[inline(always)]
    pub unsafe fn new(destinations: Destinations, offsets: Offsets) -> Self {
        Self {
            destinations,
            offsets,
        }
    }
}

impl<Destinations: SequenceLen, Offsets: SequenceLen> Graph for CSR<Destinations, Offsets>
where
    usize: To<Destinations::Item>,
    Destinations::Item: PositiveInteger,
{
    type Node = Destinations::Item;

    fn number_of_nodes(&self) -> usize {
        self.offsets.len() - 1
    }

    fn number_of_edges(&self) -> usize {
        self.destinations.len()
    }

    type Nodes = Ranger<Destinations::Item>;
    fn nodes(&self) -> Self::Nodes {
        Ranger::from_end(self.number_of_nodes().to())
    }
}

impl<Offsets: SequenceLen + SequenceRandomAccess, Destination> Successors
    for CSR<Vec<Destination>, Offsets>
where
    usize: To<Destination>,
    Destination: PositiveInteger + To<usize>,
    Offsets::Item: To<usize>,
{
    type Successors<'a> =   Copied<&'a [Destination]>
    where
        Self: 'a;

    fn successors<N: Borrow<Self::Node>>(&self, node: N) -> Self::Successors<'_> {
        let node: usize = (*node.borrow()).to();
        let start = self.offsets.get(node).to();
        let end = self.offsets.get(node + 1).to();
        Copied::new(self.destinations[start..end].as_ref())
    }

    fn has_successor<S: Borrow<Self::Node>, D: Borrow<Self::Node>>(&self, src: S, dst: D) -> bool {
        <[Destination]>::binary_search(self.successors(src).as_ref(), dst.borrow()).is_ok()
    }
}

impl<Offsets: SequenceLen + SequenceRandomAccess, Destination> Successors
    for CSR<&[Destination], Offsets>
where
    usize: To<Destination>,
    Destination: PositiveInteger + To<usize>,
    Offsets::Item: To<usize>,
{
    type Successors<'a> =   Copied<&'a [Destination]>
    where
        Self: 'a;

    fn successors<N: Borrow<Self::Node>>(&self, node: N) -> Self::Successors<'_> {
        let node: usize = (*node.borrow()).to();
        let start = self.offsets.get(node).to();
        let end = self.offsets.get(node + 1).to();
        Copied::new(self.destinations[start..end].as_ref())
    }

    fn has_successor<S: Borrow<Self::Node>, D: Borrow<Self::Node>>(&self, src: S, dst: D) -> bool {
        <[Destination]>::binary_search(self.successors(src).as_ref(), dst.borrow()).is_ok()
    }
}
