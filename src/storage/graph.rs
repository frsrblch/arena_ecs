use crate::*;
use fnv::FnvHashMap as HashMap;
use std::collections::hash_map::Entry;

#[derive(Debug, Default)]
pub struct Graph<A, W> {
    edges: HashMap<Edge<A>, W>,
    generation: AllocGen<A>,
}

impl<A, W: PartialOrd> Graph<A, W> {
    pub fn insert_min<E: ValidEdge<A>>(&mut self, edge: E, weight: W) {
        match self.edges.entry(edge.edge()) {
            Entry::Occupied(mut o) => {
                if weight < *o.get() {
                    o.insert(weight);
                }
            }
            Entry::Vacant(v) => {
                v.insert(weight);
            }
        }
    }

    pub fn insert_max<E: ValidEdge<A>>(&mut self, edge: E, weight: W) {
        match self.edges.entry(edge.edge()) {
            Entry::Occupied(mut o) => {
                if weight > *o.get() {
                    o.insert(weight);
                }
            }
            Entry::Vacant(v) => {
                v.insert(weight);
            }
        }
    }
}

impl<A, W> Graph<A, W> {
    pub fn insert<E: ValidEdge<A>>(&mut self, edge: E, weight: W) {
        self.edges.insert(edge.edge(), weight);
    }

    pub fn insert_ids<I: ValidId<A>>(&mut self, from: I, to: I, weight: W) {
        let key = Edge {
            from: from.id(),
            to: to.id(),
        };
        self.edges.insert(key, weight);
    }

    pub fn get<E: ValidEdge<A>>(&self, edge: E) -> Option<&W> {
        self.edges.get(&edge.edge())
    }

    pub fn get_mut<E: ValidEdge<A>>(&mut self, edge: E) -> Option<&mut W> {
        self.edges.get_mut(&edge.edge())
    }

    pub fn remove(&mut self, edge: Edge<A>) -> Option<W> {
        self.edges.remove(&edge)
    }

    pub fn remove_ids(&mut self, from: Id<A>, to: Id<A>) -> Option<W> {
        let key = Edge { from, to };
        self.remove(key)
    }

    pub fn clear(&mut self) {
        self.edges.clear();
    }

    pub fn get_edges_from<I: ValidId<A>>(
        &self,
        node: I,
    ) -> impl Iterator<Item = (&Edge<A>, &W)> + '_ {
        let node = node.id();
        self.edges.iter().filter(move |&(e, _)| e.from == node)
    }
}

impl<ARENA: Arena<Allocator = DynamicAllocator<ARENA>>, W> Graph<ARENA, W> {
    pub fn kill(&mut self, allocator: &Allocator<ARENA>) {
        if let Some(killed) = allocator.last_killed() {
            self.edges.retain(|edge, _| !edge.contains(killed));
            self.generation.increment();
        }
    }

    pub fn validate<'a>(&'a mut self, allocator: &'a Allocator<ARENA>) -> Valid<&'a Self> {
        self.synchronize(allocator);

        Valid::new(self)
    }

    pub fn validate_mut<'a>(&'a mut self, allocator: &'a Allocator<ARENA>) -> Valid<&'a mut Self> {
        self.synchronize(allocator);

        Valid::new(self)
    }

    fn synchronize(&mut self, allocator: &Allocator<ARENA>) {
        match allocator.generation_cmp(self.generation) {
            GenerationCmp::Valid => {}
            GenerationCmp::OffByOne(killed) => {
                self.edges.retain(|edge, _| !edge.contains(killed));
                self.generation = allocator.generation();
            }
            GenerationCmp::Outdated => {
                self.edges.retain(|edge, _| !edge.is_alive(allocator));
                self.generation = allocator.generation();
            }
        }
    }
}

impl<'a, A, W> Valid<'_, &'a Graph<A, W>> {
    pub fn iter(&'a self) -> impl Iterator<Item = (Valid<'a, &'a Edge<A>>, &W)> {
        self.value.edges.iter().map(|(e, w)| (Valid::new(e), w))
    }
}

impl<'a, A, W> Valid<'_, &'a mut Graph<A, W>> {
    pub fn iter_mut(&'a mut self) -> impl Iterator<Item = (Valid<'a, &'a Edge<A>>, &mut W)> {
        self.value.edges.iter_mut().map(|(e, w)| (Valid::new(e), w))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::allocator::test::*;

    #[test]
    fn insert_min_if_vacant() {
        let mut alloc = Allocator::<FixedArena>::default();
        let mut graph = Graph::<FixedArena, u32>::default();

        let from = alloc.create();
        let to = alloc.create();
        let edge = Edge { from, to };

        graph.insert_min(edge, 1);

        assert_eq!(Some(&1), graph.get(&edge));
    }

    #[test]
    fn insert_min_if_occupied_and_greater() {
        let mut alloc = Allocator::<FixedArena>::default();
        let mut graph = Graph::<FixedArena, u32>::default();

        let from = alloc.create();
        let to = alloc.create();
        let edge = Edge { from, to };

        graph.insert(edge, 0);

        graph.insert_min(edge, 1);

        assert_eq!(Some(&0), graph.get(&edge));
    }

    #[test]
    fn insert_min_if_occupied_and_lesser() {
        let mut alloc = Allocator::<FixedArena>::default();
        let mut graph = Graph::<FixedArena, u32>::default();

        let from = alloc.create();
        let to = alloc.create();
        let edge = Edge { from, to };

        graph.insert(edge, 2);

        graph.insert_min(edge, 1);

        assert_eq!(Some(&1), graph.get(&edge));
    }

    #[test]
    fn insert_max_if_vacant() {
        let mut alloc = Allocator::<FixedArena>::default();
        let mut graph = Graph::<FixedArena, u32>::default();

        let from = alloc.create();
        let to = alloc.create();
        let edge = Edge { from, to };

        graph.insert_max(edge, 1);

        assert_eq!(Some(&1), graph.get(&edge));
    }

    #[test]
    fn insert_max_if_occupied_and_greater() {
        let mut alloc = Allocator::<FixedArena>::default();
        let mut graph = Graph::<FixedArena, u32>::default();

        let from = alloc.create();
        let to = alloc.create();
        let edge = Edge { from, to };

        graph.insert(edge, 0);

        graph.insert_max(edge, 1);

        assert_eq!(Some(&1), graph.get(&edge));
    }

    #[test]
    fn insert_max_if_occupied_and_lesser() {
        let mut alloc = Allocator::<FixedArena>::default();
        let mut graph = Graph::<FixedArena, u32>::default();

        let from = alloc.create();
        let to = alloc.create();
        let edge = Edge { from, to };

        graph.insert(edge, 2);

        graph.insert_max(edge, 1);

        assert_eq!(Some(&2), graph.get(&edge));
    }
}
