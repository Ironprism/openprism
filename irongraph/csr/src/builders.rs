use crate::csr::CSR;
use core::marker::PhantomData;
use irontraits::{Iter, IterMut, One, PositiveInteger, SequenceAllocable, Zero};

pub struct True;
pub struct False;
pub trait Boolean {}
impl Boolean for True {}
impl Boolean for False {}

pub struct CSRBuilder<
    Offsets,
    Destinations,
    NNodes = (),
    NEdges = (),
    Sorted: Boolean = False,
    Parallel: Boolean = False,
> {
    number_of_nodes: NNodes,
    number_of_edges: NEdges,
    _marker: PhantomData<(Offsets, Destinations, Sorted, Parallel)>,
    verbose: bool,
}

impl<Offsets, Destinations> Default for CSRBuilder<Offsets, Destinations> {
    fn default() -> Self {
        CSRBuilder {
            number_of_nodes: (),
            number_of_edges: (),
            _marker: PhantomData,
            verbose: false,
        }
    }
}

impl<Offsets, Destinations, NNodes, NEdges, Sorted: Boolean, Parallel: Boolean>
    CSRBuilder<Offsets, Destinations, NNodes, NEdges, Sorted, Parallel>
{
    pub fn number_of_edges(
        self,
        number_of_edges: usize,
    ) -> CSRBuilder<Offsets, Destinations, NNodes, usize, Sorted, Parallel> {
        CSRBuilder {
            number_of_nodes: self.number_of_nodes,
            number_of_edges,
            _marker: PhantomData,
            verbose: self.verbose,
        }
    }
    pub fn number_of_nodes(
        self,
        number_of_nodes: usize,
    ) -> CSRBuilder<Offsets, Destinations, usize, NEdges, Sorted, Parallel> {
        CSRBuilder {
            number_of_nodes,
            number_of_edges: self.number_of_edges,
            _marker: PhantomData,
            verbose: self.verbose,
        }
    }
    pub fn sorted(self) -> CSRBuilder<Offsets, Destinations, NNodes, NEdges, True, Parallel> {
        CSRBuilder {
            number_of_nodes: self.number_of_nodes,
            number_of_edges: self.number_of_edges,
            _marker: PhantomData,
            verbose: self.verbose,
        }
    }
    pub fn sequential(self) -> CSRBuilder<Offsets, Destinations, NNodes, NEdges, Sorted, False> {
        CSRBuilder {
            number_of_nodes: self.number_of_nodes,
            number_of_edges: self.number_of_edges,
            _marker: PhantomData,
            verbose: self.verbose,
        }
    }
    pub fn parallel(self) -> CSRBuilder<Offsets, Destinations, NNodes, NEdges, Sorted, True> {
        CSRBuilder {
            number_of_nodes: self.number_of_nodes,
            number_of_edges: self.number_of_edges,
            _marker: PhantomData,
            verbose: self.verbose,
        }
    }
}

impl<Offsets, Destinations> CSRBuilder<Offsets, Destinations, usize, usize, True, False>
where
    Offsets: IterMut,
    Destinations: IterMut,
    <Offsets as Iter>::Item: PositiveInteger,
    <Destinations as Iter>::Item: PositiveInteger,
{
    pub fn build_from<I>(
        self,
        edges_iter: I,
        mut offsets: Offsets,
        mut destinations: Destinations,
    ) -> CSR<Destinations, Offsets>
    where
        I: IntoIterator<Item = (Destinations::Item, Destinations::Item)>,
    {
        let mut previous_src = Destinations::Item::ZERO;
        let mut current_offset = Offsets::Item::ZERO;
        let mut offsets_iter = offsets.iter_mut();
        *offsets_iter.next().unwrap() = Offsets::Item::ZERO;

        edges_iter
            .into_iter()
            .zip(destinations.iter_mut())
            .map(|((src, dst), target_dst): (_, &mut _)| {
                *target_dst = dst;
                src
            })
            .for_each(|src| {
                while previous_src < src {
                    let offset: &mut _ = offsets_iter.next().unwrap();
                    *offset = current_offset;
                    previous_src += Destinations::Item::ONE;
                }
                current_offset += Offsets::Item::ONE;
            });

        offsets_iter.for_each(|offset: &mut _| {
            *offset = current_offset;
        });

        unsafe { CSR::new(destinations, offsets) }
    }
}

impl<Offsets, Destinations> CSRBuilder<Offsets, Destinations, usize, usize, True, False>
where
    Offsets: SequenceAllocable + IterMut,
    Destinations: SequenceAllocable + IterMut,
    <Offsets as Iter>::Item: PositiveInteger,
    <Destinations as Iter>::Item: PositiveInteger,
{
    pub fn build<I>(self, edges_iter: I) -> CSR<Destinations, Offsets>
    where
        I: IntoIterator<Item = (Destinations::Item, Destinations::Item)>,
    {
        let offsets = unsafe { Offsets::uninitialized(self.number_of_nodes + 1) };
        let destinations = unsafe { Destinations::uninitialized(self.number_of_edges) };
        self.build_from(edges_iter, offsets, destinations)
    }
}
