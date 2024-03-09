use anyhow::{ensure, Context, Result};
use core::hash::{Hash, Hasher};
use epserde::{
    deser::{Deserialize, DeserializeInner, MemCase},
    traits::TypeHash,
    Epserde,
};
use graph::{Graph, Successors};
use hyperloglog_rs::prelude::*;
use irontraits::{IntoIndexedParallelIterator, Sequence, SequenceAllocable, To};
use par_replica::ParReplica;
use rayon::prelude::*;
use std::{marker::PhantomData, path::Path};
use sux::prelude::{BitFieldSlice, BitFieldSliceMut, BitFieldVec};

#[derive(Debug, Clone, Epserde)]
pub struct HyperSketchingData<Counters> {
    /// Whether to normalize the Sketching cardinalities.
    normalize: bool,
    /// How many hops
    number_of_hops: usize,
    /// the hash of the graph on which this was computed
    /// if it's Some it will be checked when converting to Hypersketching
    graph_hash: Option<u64>,
    /// Vector of HyperLogLog counters
    counters: Counters,
}

impl<Counters> AsRef<HyperSketchingData<Counters>> for HyperSketchingData<Counters> {
    #[inline(always)]
    fn as_ref(&self) -> &HyperSketchingData<Counters> {
        self
    }
}

pub struct HyperSketching<G, Data, Counters, P = Precision12, const BITS: usize = 6>
where
    P: Precision + WordType<BITS>,
    Counters: Sequence<Item = HyperLogLog<P, BITS>>,
{
    /// The portion of hypersketching that can be serialized and deserialized
    data: Data,
    /// The support graph to use.
    graph: G,
    /// Masks to be used by the various threads without reallocation.
    masks: Option<ParReplica<BitFieldVec>>,

    _marker: PhantomData<Counters>,
}

impl<G, Counters, P, const BITS: usize>
    HyperSketching<G, HyperSketchingData<Counters>, Counters, P, BITS>
where
    G: Graph + Hash,
    Counters: SequenceAllocable<Item = HyperLogLog<P, BITS>>,
    P: Precision + WordType<BITS>,
{
    /// Creates a new HyperSketching model.
    ///
    /// # Arguments
    /// * `number_of_hops`: usize - The number of hops to use for the Sketching.
    /// * `normalize`: bool - Whether to normalize the Sketching cardinalities. By default, false.
    /// * `graph`: Graph - The graph whose edges are to be learned.
    ///
    pub fn new(number_of_hops: usize, normalize: bool, graph: G) -> Result<Self> {
        ensure!(
            number_of_hops > 0,
            "The number of hops must be greater than 0."
        );
        Ok(Self {
            data: HyperSketchingData {
                counters: Counters::empty(),
                number_of_hops,
                normalize,
                graph_hash: None,
            },
            masks: None,
            graph,
            _marker: PhantomData,
        })
    }

    pub unsafe fn from_data<Data>(
        data: Data,
        graph: G,
        check_graph_hash: bool,
    ) -> Result<HyperSketching<G, Data, Counters, P, BITS>>
    where
        Data: AsRef<HyperSketchingData<Counters>>,
    {
        if check_graph_hash {
            if let Some(graph_hash) = data.as_ref().graph_hash {
                let mut hasher = std::collections::hash_map::DefaultHasher::new();
                graph.hash(&mut hasher);
                ensure!(
                    graph_hash == hasher.finish(),
                    "The graph hash does not match the provided graph."
                );
            }
        }

        Ok(HyperSketching {
            data,
            graph,
            masks: None,
            _marker: PhantomData,
        })
    }
}

impl<G, Counters, P, const BITS: usize>
    HyperSketching<G, HyperSketchingData<Counters>, Counters, P, BITS>
where
    G: Graph + Hash,
    Counters: SequenceAllocable<Item = HyperLogLog<P, BITS>> + Deserialize + TypeHash,
    P: Precision + WordType<BITS>,
{
    pub unsafe fn load<PP>(
        path: PP,
        graph: G,
        check_graph_hash: bool,
    ) -> Result<HyperSketching<G, HyperSketchingData<Counters>, Counters, P, BITS>>
    where
        PP: AsRef<Path>,
    {
        let data = <HyperSketchingData<Counters>>::load_full(path.as_ref()).with_context(|| {
            format!(
                "Error while loading hypersketching data from {}",
                path.as_ref().display()
            )
        })?;
        Self::from_data(data, graph, check_graph_hash)
    }
}

type DeserType<Counters> =
    MemCase<<HyperSketchingData<Counters> as DeserializeInner>::DeserType<'static>>;

impl<G, Counters, P, const BITS: usize>
    HyperSketching<G, HyperSketchingData<Counters>, Counters, P, BITS>
where
    G: Graph + Hash,
    Counters: SequenceAllocable<Item = HyperLogLog<P, BITS>> + Deserialize + TypeHash,
    P: Precision + WordType<BITS>,
    DeserType<Counters>: AsRef<HyperSketchingData<Counters>>,
{
    pub unsafe fn mmap<PP>(
        path: PP,
        graph: G,
        check_graph_hash: bool,
    ) -> Result<HyperSketching<G, DeserType<Counters>, Counters, P, BITS>>
    where
        PP: AsRef<Path>,
    {
        let data = <HyperSketchingData<Counters>>::mmap::<'static>(
            path.as_ref(),
            epserde::deser::Flags::RANDOM_ACCESS,
        )
        .with_context(|| {
            format!(
                "Error while loading hypersketching data from {}",
                path.as_ref().display()
            )
        })?;
        Self::from_data(data, graph, check_graph_hash)
    }

    /*
    ParReplica::new(BitFieldVec::new(
                ,
                graph.number_of_nodes(),
            ))
    */
}

impl<G, Data, Counters, P, const BITS: usize> HyperSketching<G, Data, Counters, P, BITS>
where
    G: Graph,
    Counters: Sequence<Item = HyperLogLog<P, BITS>>,
    P: Precision + WordType<BITS>,
    Data: AsRef<HyperSketchingData<Counters>>,
{
    /// Returns the number of hops.
    #[inline(always)]
    pub fn number_of_hops(&self) -> usize {
        self.data.as_ref().number_of_hops
    }

    #[inline(always)]
    fn bits_per_node(&self) -> usize {
        (self.number_of_hops() as f32 + 2.0).log2().ceil() as usize
    }

    /// Returns the not visited const specific to the provided number of hops.
    ///
    /// # Implementation details
    /// This value is equal to the maximum value that can be represented
    /// by the number of bits necessary to store the number of hops plus 2.
    /// We need the plus two as one of the values is used to represent
    /// the deleted nodes and the other one is used to represent the
    /// not visited nodes.
    #[inline(always)]
    fn not_visited(&self) -> usize {
        (1 << self.bits_per_node()) - 1
    }

    #[inline(always)]
    fn deleted(&self) -> usize {
        self.not_visited() - 1
    }

    #[inline(always)]
    pub fn normalize(&self) -> bool {
        self.data.as_ref().normalize
    }
}

impl<G, Data, Counters, P, const BITS: usize> HyperSketching<G, Data, Counters, P, BITS>
where
    G: Graph,
    Counters: Sequence<Item = HyperLogLog<P, BITS>>,
    P: Precision + WordType<BITS>,
    Data: AsMut<HyperSketchingData<Counters>>,
{
    #[inline(always)]
    pub fn set_normalize(&mut self, normalize: bool) {
        self.data.as_mut().normalize = normalize;
    }
}

impl<G, Data, Counters, P, const BITS: usize> HyperSketching<G, Data, Counters, P, BITS>
where
    G: Successors + Send + Sync,
    <G as Graph>::Nodes: IntoIndexedParallelIterator<Item = G::Node>,
    Counters: Send
        + Sync
        + SequenceAllocable<Item = HyperLogLog<P, BITS>>
        + AsRef<[HyperLogLog<P, BITS>]>
        + AsMut<[HyperLogLog<P, BITS>]>,
    P: Precision + WordType<BITS>,
    Data: AsRef<HyperSketchingData<Counters>> + AsMut<HyperSketchingData<Counters>>,
{
    /// Fit the HyperBall model to the provided support.
    pub fn fit(&mut self) {
        let number_of_hops = self.number_of_hops();
        let number_of_counters = self.graph.number_of_nodes() * number_of_hops;
        let data = self.data.as_mut();
        data.counters = Counters::defaulted(number_of_counters);

        // Create HyperLogLog counters for all nodes in the graph
        self.graph
            .nodes()
            .into_par_iter()
            .zip(
                data.counters
                    .as_mut()
                    .par_chunks_exact_mut(number_of_hops)
                    .map(|counters| &mut counters[0]),
            )
            .for_each(|(node, hll): (G::Node, &mut HyperLogLog<P, BITS>)| {
                hll.insert(node);
                hll.bitor_assign(self.graph.successors(node));
            });

        // Iterate over all hops and update the counters accordingly
        (1..number_of_hops).for_each(|k| {
            self.graph
                .nodes()
                .into_par_iter()
                .zip(
                    data.counters
                        .as_ref()
                        .par_chunks_exact(number_of_hops)
                        .map(|row| {
                            let (previous, current) = row.split_at(k);
                            let current = unsafe {
                                core::mem::transmute_copy::<_, &mut [HyperLogLog<P, BITS>]>(
                                    &current,
                                )
                            };
                            (&previous[k - 1], &mut current[0])
                        }),
                )
                .for_each(|(node, (previous_counter, current_counter))| {
                    // Iterate over all neighbors of the current node
                    *current_counter = self
                        .graph
                        .successors(node)
                        .into_iter()
                        .map(|dst| data.counters.as_ref()[dst.to() * number_of_hops + k - 1])
                        .union()
                        | previous_counter;
                });
        });
    }
}

#[derive(Debug, Clone)]
struct Normalized {
    left_cardinality: Vec<f32>,
    right_cardinality: Vec<f32>,
}

#[derive(Debug, Clone)]
struct NotNormalized;

trait Normalizer {
    fn new(number_of_hops: usize) -> Self;

    fn inc_left_cardinality(&mut self, hop: usize, inc: f32);
    fn inc_right_cardinality(&mut self, hop: usize, inc: f32);

    fn normalize(
        &mut self,
        intersections: &mut [f32],
        left_difference: &mut [f32],
        right_difference: &mut [f32],
    );
}

impl Normalizer for Normalized {
    #[inline(always)]
    fn new(number_of_hops: usize) -> Self {
        Self {
            left_cardinality: vec![0.0; number_of_hops],
            right_cardinality: vec![0.0; number_of_hops],
        }
    }

    #[inline(always)]
    fn inc_left_cardinality(&mut self, hop: usize, inc: f32) {
        self.left_cardinality[hop] += inc;
    }

    #[inline(always)]
    fn inc_right_cardinality(&mut self, hop: usize, inc: f32) {
        self.right_cardinality[hop] += inc;
    }

    #[inline(always)]
    fn normalize(
        &mut self,
        intersections: &mut [f32],
        left_difference: &mut [f32],
        right_difference: &mut [f32],
    ) {
        let hops = self.left_cardinality.len();
        for i in 0..hops {
            let mut current_left_difference = left_difference[i];
            for j in (0..hops).rev() {
                let non_normalized_differential_intersection = intersections[i * hops + j];
                intersections[i * hops + j] /=
                    (current_left_difference + self.right_cardinality[j]).max(f32::EPSILON);
                current_left_difference += non_normalized_differential_intersection;
            }
            left_difference[i] /= self.left_cardinality[i].max(f32::EPSILON);
            right_difference[i] /= self.right_cardinality[i].max(f32::EPSILON);
        }
    }
}

impl Normalizer for NotNormalized {
    #[inline(always)]
    fn new(_number_of_hops: usize) -> Self {
        Self
    }

    #[inline(always)]
    fn inc_left_cardinality(&mut self, _hop: usize, _inc: f32) {}

    #[inline(always)]
    fn inc_right_cardinality(&mut self, _hop: usize, _inc: f32) {}

    #[inline(always)]
    fn normalize(
        &mut self,
        _intersections: &mut [f32],
        _left_difference: &mut [f32],
        _right_difference: &mut [f32],
    ) {
    }
}

impl<G, Data, Counters, P, const BITS: usize> HyperSketching<G, Data, Counters, P, BITS>
where
    G: Successors + Send + Sync,
    <G as Graph>::Nodes: IntoIndexedParallelIterator<Item = G::Node>,
    Counters: Send
        + Sync
        + SequenceAllocable<Item = HyperLogLog<P, BITS>>
        + AsRef<[HyperLogLog<P, BITS>]>
        + AsMut<[HyperLogLog<P, BITS>]>,
    P: Precision + WordType<BITS>,
    Data: AsRef<HyperSketchingData<Counters>> + AsMut<HyperSketchingData<Counters>>,
{

    #[inline(always)]
    pub fn biased_edge_features<const INSERT_EDGE: bool>(
        &self,
        (src, dst): (G::Node, G::Node),
        target: &mut [f32],
    ) {
        // We get the usize representation of the nodes.
        let src: usize = src.to();
        let dst: usize = dst.to();

        // Now, we can compute the overlap matrix.
        if self.normalize() {
            let (overlaps, left_diffs, right_diffs) = <HyperLogLog<P, BITS> as HyperSpheresSketch<f32>>::normalized_overlap_and_differences_cardinality_matrices_vec(
                &self.data.as_ref().counters.as_ref()[src * self.number_of_hops()..(src + 1) * self.number_of_hops()],
                &self.data.as_ref().counters.as_ref()[dst * self.number_of_hops()..(dst + 1) * self.number_of_hops()],
            );
            target
            .iter_mut()
            .zip(
                overlaps
                    .into_iter()
                    .flat_map(|o| o)
                    .chain(left_diffs)
                    .chain(right_diffs),
            )
            .for_each(|(t, v)| *t = v);
        } else {
            let (overlaps, left_diffs, right_diffs) = <HyperLogLog<P, BITS> as HyperSpheresSketch<f32>>::overlap_and_differences_cardinality_matrices_vec(
                &self.data.as_ref().counters.as_ref()[src * self.number_of_hops()..(src + 1) * self.number_of_hops()],
                &self.data.as_ref().counters.as_ref()[dst * self.number_of_hops()..(dst + 1) * self.number_of_hops()],
            );
            target
            .iter_mut()
            .zip(
                overlaps
                    .into_iter()
                    .flat_map(|o| o)
                    .chain(left_diffs)
                    .chain(right_diffs),
            )
            .for_each(|(t, v)| *t = v.round());
        };
    }

    #[inline(always)]
    pub fn bias_aware_edge_features<const INSERT_EDGE: bool>(
        &self,
        (src, dst): (G::Node, G::Node),
        target: &mut [f32],
    ) {
        if self.data.as_ref().normalize {
            self.bias_aware_edge_features_dispatched::<INSERT_EDGE, Normalized>((src, dst), target)
        } else {
            self.bias_aware_edge_features_dispatched::<INSERT_EDGE, NotNormalized>(
                (src, dst),
                target,
            )
        }
    }

    fn bias_aware_edge_features_dispatched<const INSERT_EDGE: bool, Norm: Normalizer>(
        &self,
        (src, dst): (G::Node, G::Node),
        target: &mut [f32],
    ) {
        let mask = unsafe {
            self.masks
                .as_ref()
                .expect("bias_aware_edge_features called before fitting")
                .get_mut()
        };
        mask.reset_ones();

        let hops = self.number_of_hops();
        let mut normalizer = Norm::new(hops);

        // We check that the provided target features are all zero.
        debug_assert!(
            target.iter().all(|v| *v == 0.0),
            "The provided target features must be all zero."
        );

        let (differential_intersections, differences) = target.split_at_mut(hops * hops);

        debug_assert_eq!(differential_intersections.len(), hops * hops);

        let (left_difference, right_difference) = differences.split_at_mut(hops);

        debug_assert_eq!(left_difference.len(), hops);

        debug_assert_eq!(right_difference.len(), hops);

        // First, we work on the source node.

        let mut frontier: Vec<G::Node> = Vec::new();
        let mut temporary_frontier = Vec::new();
        let last_hop = hops - 1;

        let skip_edge = |v: G::Node, w: G::Node| -> bool {
            (v == src && w == dst) || (self.graph.undirected() && v == dst && w == src)
        };

        let mut count = |w: G::Node, i: usize| -> bool {
            let w_usize: usize = w.to();
            let w_hops: usize = unsafe { mask.get_unchecked(w_usize) };
            let not_visited = w_hops == self.not_visited();

            left_difference[i] += not_visited as usize as f32;
            normalizer.inc_left_cardinality(i, not_visited as usize as f32);
            unsafe {
                mask.set_unchecked(w_usize, core::cmp::min(w_hops, i));
            }
            not_visited
        };

        count(src, 0);
        frontier.push(src);

        // Then, we populate the hypersphere of neighbours up to the given number of hops.
        for i in 0..(hops - 1) {
            for v in frontier.drain(..) {
                for w in self.graph.successors(v) {
                    if !skip_edge(v, w) && count(w, i) {
                        temporary_frontier.push(w);
                    }
                }

                if INSERT_EDGE && v == src && count(dst, i) {
                    temporary_frontier.push(dst);
                }
            }
            // We swap the frontiers.
            std::mem::swap(&mut frontier, &mut temporary_frontier);
        }

        if INSERT_EDGE && hops == 1 {
            count(dst, 0);
        }

        for v in frontier.drain(..) {
            for w in self.graph.successors(v) {
                if !skip_edge(v, w) {
                    count(w, last_hop);
                }
            }
        }

        let mut count = |w: G::Node, i: usize| -> bool {
            let w_usize: usize = w.to();
            let w_hops: usize = unsafe { mask.get_unchecked(w_usize) };
            let not_visited = w_hops == self.not_visited();
            let not_deleted = w_hops != self.deleted();

            if !not_visited && not_deleted {
                differential_intersections[w_hops * hops + i] += 1.0;
                left_difference[w_hops] -= 1.0;
            }
            right_difference[i] += not_visited as usize as f32;
            normalizer.inc_right_cardinality(i, not_deleted as usize as f32);

            unsafe {
                mask.set_unchecked(w_usize, self.deleted());
            }

            not_deleted
        };

        count(dst, 0);
        frontier.push(dst);

        for i in 0..(hops - 1) {
            for v in frontier.drain(..) {
                for w in self.graph.successors(v) {
                    if !skip_edge(v, w) && count(w, i) {
                        temporary_frontier.push(w);
                    }
                }

                if INSERT_EDGE {
                    if v == src && count(dst, i) {
                        temporary_frontier.push(dst);
                    } else if self.graph.undirected() && v == dst && count(src, i) {
                        temporary_frontier.push(src);
                    }
                }
            }

            // We swap the frontiers.
            std::mem::swap(&mut frontier, &mut temporary_frontier);
        }

        if INSERT_EDGE && self.graph.undirected() && hops == 1 {
            count(src, 0);
        }

        for v in frontier.drain(..) {
            for w in self.graph.successors(v) {
                if !skip_edge(v, w) {
                    count(w, last_hop);
                }
            }
        }

        normalizer.normalize(
            differential_intersections,
            left_difference,
            right_difference,
        );
    }
}
