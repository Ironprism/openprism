//! Module providing a builder for the compressed sparse row (CSR) data structure.
use crate::prelude::*;
use graph::array_like::IterMut;
use indicatif::{ProgressBar, ProgressStyle};

// n nodes vs not
// n edges vs not
// sorted vs not
// sequential vs parallel


pub struct CSRBuilder {
    number_of_nodes: Option<usize>,
    number_of_edges: Option<usize>,
    sorted: bool,
    verbose: bool,
}

pub enum Error {}

impl CSRBuilder {
    pub fn number_of_nodes(&mut self, number_of_nodes: usize) -> &mut Self {
        self.number_of_nodes = Some(number_of_nodes);
        self
    }

    pub fn number_of_edges(&mut self, number_of_edges: usize) -> &mut Self {
        self.number_of_edges = Some(number_of_edges);
        self
    }

    pub fn sorted(&mut self, sorted: bool) -> &mut Self {
        self.sorted = Some(sorted);
        self
    }

    pub fn verbose(&mut self, verbose: bool) -> &mut Self {
        self.verbose = verbose;
        self
    }
}

impl CSRBuilder {
    fn build_sorted_sequential_sized<Destinations, Offsets, ES>(
        self,
        edge_stream: ES,
        number_of_nodes: usize,
        number_of_edges: usize,
    ) -> CSR<Destinations, Offsets>
    where
        ES: IntoIterator<
            Item = (
                <Destinations as ArrayLike>::Item,
                <Destinations as ArrayLike>::Item,
            ),
        >,
        Destinations: ArrayLike + IterMut<Item = <Destinations as ArrayLike>::Item>,
        Offsets: ArrayLike,
        <Destinations as ArrayLike>::Item: PositiveInteger,
        <Offsets as ArrayLike>::Item: PositiveInteger,
    {
        let mut destinations = unsafe { Destinations::uninitialized(number_of_edges) };
        let mut offsets = Offsets::zeroed(number_of_nodes + 1);
        let mut current_node: <Destinations as ArrayLike>::Item =
            <Destinations as ArrayLike>::Item::ZERO;
        let mut current_node_usize: usize = 0_usize;
        let mut current_offset: Offsets::Item = Offsets::Item::ZERO;

        for ((src, dst), target_dst) in edge_stream.into_iter().zip(destinations.iter_mut()) {
            while current_node < src {
                offsets[current_node_usize] = current_offset;
                current_node = current_node + <Destinations as ArrayLike>::Item::ONE;
            }

            *target_dst = dst;
            current_offset += Offsets::Item::ONE;
            current_node_usize += 1;
        }

        while current_node_usize <= number_of_nodes {
            offsets[current_node_usize] = current_offset;
            current_node_usize += 1;
        }

        CSR::new(destinations, offsets)
    }

    fn build_sorted_sequential<Destinations, Offsets, ES>(
        self,
        edge_stream: ES,
    ) -> CSR<Destinations, Offsets>
    where
        ES: IntoIterator<
            Item = (
                <Destinations as VectorLike>::Item,
                <Destinations as VectorLike>::Item,
            ),
        >,
        Destinations: VectorLike + IterMut<Item = <Destinations as VectorLike>::Item>,
        Offsets: VectorLike,
        <Destinations as VectorLike>::Item: PositiveInteger,
        <Offsets as VectorLike>::Item: PositiveInteger,
    {
        let mut destinations = Destinations::empty();
        let mut offsets = Offsets::empty();
        let mut current_node: <Destinations as ArrayLike>::Item =
            <Destinations as ArrayLike>::Item::ZERO;
        let mut current_offset: Offsets::Item = Offsets::Item::ZERO;
        let mut max_dst = <Destinations as ArrayLike>::Item::ZERO;

        for ((src, dst), target_dst) in edge_stream.into_iter().zip(destinations.iter_mut()) {
            while current_node < src {
                offsets.push(current_offset);
                current_node = current_node + <Destinations as ArrayLike>::Item::ONE;
            }

            *target_dst = dst;
            max_dst = max_dst.max(dst);
            current_offset += Offsets::Item::ONE;
        }

        while offsets.len() <= max_dst {
            offsets.push(current_offset);
        }

        CSR::new(destinations, offsets)
    }

    fn sorted_sized_sequential<Destinations, Offsets, ES>(
        self,
        edge_stream: ES,
    ) -> Result<CSR<Destinations, Offsets>, Error>
    where
        ES: IntoIterator<
            Item = (
                <Destinations as ArrayLike>::Item,
                <Destinations as ArrayLike>::Item,
            ),
        >,
    {
        match (self.number_of_edges, self.number_of_nodes) {
            (None, None) => Ok(self.build_sorted_sequential(edge_stream)),
            (Some(number_of_edges), None) => {
                unimplemented!("Case Some None not implemented yet")
            }
            (None, Some(number_of_nodes)) => {
                unimplemented!("Case None Some not implemented yet")
            }
            (Some(number_of_edges), Some(number_of_nodes)) => Ok(
                self.build_sorted_sequential_sized(edge_stream, number_of_nodes, number_of_edges)
            ),
        }
    }

    fn split_sorted_sequential<Destinations, Offsets, ES>(
        self,
        edge_stream: ES,
    ) -> Result<CSR<Destinations, Offsets>, Error>
    where
        ES: IntoIterator<
            Item = (
                <Destinations as ArrayLike>::Item,
                <Destinations as ArrayLike>::Item,
            ),
        >,
    {
        if self.sorted.or(false) {
            unimplemented!("We have not yet implemented the unsorted case.")
        } else {
            self.sorted_sized_sequential(edge_stream)
        }
    }

    fn build_verbose_sequential<Destinations, Offsets, ES>(
        self,
        edge_stream: ES,
    ) -> Result<CSR<Destinations, Offsets>, Error>
    where
        ES: IntoIterator<
            Item = (
                <Destinations as ArrayLike>::Item,
                <Destinations as ArrayLike>::Item,
            ),
        >,
    {
        let batches_progress_bar = ProgressBar::hidden();
        batches_progress_bar.set_style(
            ProgressStyle::default_bar()
                .template(
                    "Batches: {msg} [{elapsed_precise}] {wide_bar:40.cyan/blue} {pos:>7}/{len:7}",
                )
                .unwrap()
                .progress_chars("##-"),
        );
        self.split_sorted(edge_stream.into_iter().progress_with(batches_progress_bar))
    }

    // Build the CSR from an edge stream.
    pub fn build<Destinations, Offsets, ES>(
        self,
        edge_stream: ES,
    ) -> Result<CSR<Destinations, Offsets>, Error>
    where
        ES: IntoIterator<
            Item = (
                <Destinations as ArrayLike>::Item,
                <Destinations as ArrayLike>::Item,
            ),
        >,
    {
        if self.verbose {
            self.build_verbose(edge_stream)
        } else {
            self.split_sorted(edge_stream)
        }
    }
}
