use crate::Successors;
use ironstructs::ranger::Ranger;
use irontraits::{To, Zero};

pub struct CoordinatesIter<'a, G: Successors> {
    graph: &'a G,
    nodes: Ranger<G::Node>,
    edges: Ranger<usize>,
    successors: <<G as Successors>::Successors<'a> as IntoIterator>::IntoIter,
    end_successors: <<G as Successors>::Successors<'a> as IntoIterator>::IntoIter,
}

impl<'a, G: Successors> From<&'a G> for CoordinatesIter<'a, G>
where
    usize: To<G::Node>,
{
    fn from(graph: &'a G) -> Self {
        CoordinatesIter {
            graph,
            nodes: Ranger::from_end(graph.number_of_nodes().to()),
            edges: Ranger::from_end(graph.number_of_edges()),
            successors: graph.successors(G::Node::ZERO).into_iter(),
            end_successors: graph.successors((graph.number_of_nodes() - 1).to()).into_iter(),
        }
    }
}

impl<'a, G: Successors> Iterator for CoordinatesIter<'a, G>
where
    usize: To<G::Node>,
{
    type Item = (G::Node, G::Node);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(dst) = self.successors.next() {
            Some((self.nodes.start(), dst))
        } else {
            match self.nodes.next() {
                Some(node) => {
                    self.successors = self.graph.successors(node).into_iter();
                    self.next()
                }
                None => self
                    .end_successors
                    .next()
                    .map(|dst| (self.nodes.end(), dst)),
            }
        }
    }
}

impl<'a, G: Successors> DoubleEndedIterator for CoordinatesIter<'a, G>
where
    usize: To<G::Node>,
    <G::Successors<'a> as IntoIterator>::IntoIter: DoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if let Some(dst) = self.end_successors.next_back() {
            Some((self.nodes.end(), dst))
        } else {
            match self.nodes.next_back() {
                Some(node) => {
                    self.end_successors = self.graph.successors(node).into_iter();
                    self.next_back()
                }
                None => self
                    .successors
                    .next_back()
                    .map(|dst| (self.nodes.start(), dst)),
            }
        }
    }
}

impl<'a, G: Successors> ExactSizeIterator for CoordinatesIter<'a, G>
where
    usize: To<G::Node>,
{
    fn len(&self) -> usize {
        self.edges.len()
    }
}
