use crate::csr::CSR;
use graph::Graph;
use ironstructs::ranger::Ranger;
use irontraits::{Iter, SequenceLen, SequenceRandomAccess, To};

pub struct CSRCoordinatesIter<'a, Destinations: Iter, Offsets: Iter>
where
    CSR<Destinations, Offsets>: Graph<Node = Destinations::Item>,
{
    pub(crate) csr: &'a CSR<Destinations, Offsets>,
    pub(crate) nodes: Ranger<usize>,
    pub(crate) edges: Ranger<usize>,
}

impl<'a, Destinations: SequenceLen, Offsets: SequenceLen> From<&'a CSR<Destinations, Offsets>>
    for CSRCoordinatesIter<'a, Destinations, Offsets>
where
    CSR<Destinations, Offsets>: Graph<Node = Destinations::Item>,
{
    fn from(csr: &'a CSR<Destinations, Offsets>) -> Self {
        CSRCoordinatesIter {
            csr,
            nodes: Ranger::from_end(csr.number_of_nodes()),
            edges: Ranger::from_end(csr.number_of_edges()),
        }
    }
}

impl<'a, Destinations: Iter + SequenceRandomAccess, Offsets: Iter + SequenceRandomAccess> Iterator
    for CSRCoordinatesIter<'a, Destinations, Offsets>
where
    CSR<Destinations, Offsets>: Graph<Node = Destinations::Item>,
    usize: To<Destinations::Item>,
    Offsets::Item: To<usize>,
{
    type Item = (Destinations::Item, Destinations::Item);

    fn next(&mut self) -> Option<Self::Item> {
        let edge = self.edges.next()?;
        loop {
            if self.nodes.start() > self.nodes.end() {
                return None;
            }

            if self.csr.offsets.get(self.nodes.start() + 1).to() <= edge {
                self.nodes.next()?;
                continue;
            }

            let dst = self.csr.destinations.get(edge);
            return Some((self.nodes.start().to(), dst));
        }
    }
}

impl<'a, Destinations: Iter + SequenceRandomAccess, Offsets: Iter + SequenceRandomAccess>
    DoubleEndedIterator for CSRCoordinatesIter<'a, Destinations, Offsets>
where
    CSR<Destinations, Offsets>: Graph<Node = Destinations::Item>,
    usize: To<Destinations::Item>,
    Offsets::Item: To<usize>,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let edge = self.edges.next_back()?;
        loop {
            if self.nodes.start() > self.nodes.end() {
                return None;
            }

            if self.csr.offsets.get(self.nodes.end()).to() > edge {
                self.nodes.next_back()?;
                continue;
            }

            let dst = self.csr.destinations.get(edge);
            return Some((self.nodes.end().to(), dst));
        }
    }
}

impl<'a, Destinations: Iter + SequenceRandomAccess, Offsets: Iter + SequenceRandomAccess>
    ExactSizeIterator for CSRCoordinatesIter<'a, Destinations, Offsets>
where
    CSR<Destinations, Offsets>: Graph<Node = Destinations::Item>,
    usize: To<Destinations::Item>,
    Offsets::Item: To<usize>,
{
    #[inline(always)]
    fn len(&self) -> usize {
        self.edges.len()
    }
}
