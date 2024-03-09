use crate::csr::CSR;
use crate::iter::CSRCoordinatesIter;
use graph::Graph;
use ironstructs::ranger::Ranger;
use irontraits::{Iter, SequenceRandomAccess, To};
use rayon::iter::plumbing::*;
use rayon::prelude::*;

impl<'a, Destinations, Offsets> Producer for CSRCoordinatesIter<'a, Destinations, Offsets>
where
    Destinations: Iter + SequenceRandomAccess + Send + Sync,
    Offsets: Iter + SequenceRandomAccess + Send + Sync,
    CSR<Destinations, Offsets>: Graph<Node = Destinations::Item>,
    usize: To<Destinations::Item> + To<Offsets::Item>,
    Offsets::Item: To<usize> + Ord,
{
    type Item = (Destinations::Item, Destinations::Item);
    type IntoIter = Self;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self
    }

    #[inline(always)]
    fn split_at(self, index: usize) -> (Self, Self) {
        let (left_edges, right_edges) = self.edges.split_at(index);

        // find the src
        let split_node = right_edges.start().to();
        let src = self
            .csr
            .offsets
            .partition_point(|x| x <= split_node, self.nodes.clone().into())
            - 1;

        (
            CSRCoordinatesIter {
                csr: self.csr,
                nodes: Ranger::new(self.nodes.start(), src),
                edges: left_edges,
            },
            CSRCoordinatesIter {
                csr: self.csr,
                nodes: Ranger::new(src, self.nodes.end()),
                edges: right_edges,
            },
        )
    }
}

impl<'a, Destinations, Offsets> ParallelIterator for CSRCoordinatesIter<'a, Destinations, Offsets>
where
    Destinations: Iter + SequenceRandomAccess + Send + Sync,
    Offsets: Iter + SequenceRandomAccess + Send + Sync,
    CSR<Destinations, Offsets>: Graph<Node = Destinations::Item>,
    usize: To<Destinations::Item> + To<Offsets::Item>,
    Offsets::Item: To<usize> + Ord,
    Destinations::Item: Send + Sync,
{
    type Item = (Destinations::Item, Destinations::Item);

    #[inline(always)]
    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        bridge(self, consumer)
    }
}

impl<'a, Destinations, Offsets> IndexedParallelIterator
    for CSRCoordinatesIter<'a, Destinations, Offsets>
where
    Destinations: Iter + SequenceRandomAccess + Send + Sync,
    Offsets: Iter + SequenceRandomAccess + Send + Sync,
    CSR<Destinations, Offsets>: Graph<Node = Destinations::Item>,
    usize: To<Destinations::Item> + To<Offsets::Item>,
    Offsets::Item: To<usize> + Ord,
    Destinations::Item: Send + Sync,
{
    #[inline(always)]
    fn len(&self) -> usize {
        ExactSizeIterator::len(&self.edges)
    }

    #[inline(always)]
    fn drive<C>(self, consumer: C) -> C::Result
    where
        C: Consumer<Self::Item>,
    {
        bridge(self, consumer)
    }

    #[inline(always)]
    fn with_producer<CB>(self, callback: CB) -> CB::Output
    where
        CB: ProducerCallback<Self::Item>,
    {
        callback.callback(self)
    }
}
