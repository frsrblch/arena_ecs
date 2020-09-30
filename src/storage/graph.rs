use crate::*;
use fnv::FnvHashMap as HashMap;
use std::collections::hash_map::Entry;

#[derive(Debug, Default)]
pub struct Graph<A, W> {
    edges: HashMap<Edge<A>, W>,
    generation: u64,
}

impl<A, W: PartialOrd> Graph<A, W> {
    pub fn insert_min<I: Indexes<A>>(&mut self, from: I, to: I, weight: W) {
        let key = Edge { from: from.id(), to: to.id() };

        match self.edges.entry(key) {
            Entry::Occupied(mut o) => {
                if weight < *o.get() {
                    o.insert(weight);
                }
            },
            Entry::Vacant(v) => {
                v.insert(weight);
            },
        }
    }

    pub fn insert_max<I: Indexes<A>>(&mut self, from: I, to: I, weight: W) {
        let key = Edge { from: from.id(), to: to.id() };

        match self.edges.entry(key) {
            Entry::Occupied(mut o) => {
                if weight > *o.get() {
                    o.insert(weight);
                }
            },
            Entry::Vacant(v) => {
                v.insert(weight);
            },
        }
    }
}

impl<A, W> Graph<A, W> {
    pub fn insert_valid(&mut self, edge: Valid<Edge<A>>, weight: W) {
        self.edges.insert(edge.value, weight);
    }

    pub fn insert_ids<I: Indexes<A>>(&mut self, from: I, to: I, weight: W) {
        let key = Edge { from: from.id(), to: to.id() };
        self.edges.insert(key, weight);
    }

    pub fn insert(&mut self, edge: Edge<A>, weight: W) {
        self.edges.insert(edge, weight);
        self.generation = 0;
    }

    pub fn get(&self, edge: &Edge<A>) -> Option<&W> {
        self.edges.get(edge)
    }

    pub fn get_mut(&mut self, edge: &Edge<A>) -> Option<&mut W> {
        self.edges.get_mut(edge)
    }

    pub fn remove(&mut self, from: Id<A>, to: Id<A>) -> Option<W> {
        let key = Edge { from, to };
        self.edges.remove(&key)
    }

    pub fn clear(&mut self) {
        self.edges.clear();
    }

    pub fn get_edges<I: Indexes<A>>(&self, node: I) -> impl Iterator<Item=(&Edge<A>, &W)> + '_ {
        let node = node.id();
        self.edges
            .iter()
            .filter(move |&(e, _)| e.from == node)
    }
}

impl<A: Arena<Allocator=DynamicAllocator<A>>, W> Graph<A, W> {
    pub fn kill(&mut self, id: Id<A>) {
        self.edges.retain(|edge, _| !edge.contains(id));

        self.generation += 1;
    }

    pub fn retain_living(&mut self, allocator: &Allocator<A>) {
        self.edges.retain(|edge, _| edge.is_alive(allocator));
    }

    pub fn validate<'a>(&'a mut self, allocator: &'a Allocator<A>) -> Valid<&'a Self> {
        if !self.is_synchronized(allocator) {
            eprintln!("Unsynchronized graph: {}", std::any::type_name::<Self>());
            self.retain_living(allocator);
        }

        Valid::new(self)
    }

    pub fn validate_mut<'a>(&'a mut self, allocator: &'a Allocator<A>) -> Valid<&'a mut Self> {
        if !self.is_synchronized(allocator) {
            eprintln!("Unsynchronized graph: {}", std::any::type_name::<Self>());
            self.retain_living(allocator);
        }

        Valid::new(self)
    }

    pub fn try_validate<'a>(&'a self, allocator: &'a Allocator<A>) -> Option<Valid<&'a Self>> {
        if self.is_synchronized(allocator) {
            Some(Valid::new(self))
        } else {
            eprintln!("Unsynchronized graph: {}", std::any::type_name::<Self>());
            None
        }
    }

    fn is_synchronized(&self, allocator: &Allocator<A>) -> bool {
        self.generation == allocator.generation()
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

        graph.insert_min(from, to, 1);

        assert_eq!(Some(&1), graph.get(&edge));
    }

    #[test]
    fn insert_min_if_occupied_and_greater() {
        let mut alloc = Allocator::<FixedArena>::default();
        let mut graph = Graph::<FixedArena, u32>::default();

        let from = alloc.create();
        let to = alloc.create();
        let edge = Edge { from, to };

        graph.insert_ids(from, to, 0);

        graph.insert_min(from, to, 1);

        assert_eq!(Some(&0), graph.get(&edge));
    }

    #[test]
    fn insert_min_if_occupied_and_lesser() {
        let mut alloc = Allocator::<FixedArena>::default();
        let mut graph = Graph::<FixedArena, u32>::default();

        let from = alloc.create();
        let to = alloc.create();
        let edge = Edge { from, to };

        graph.insert_ids(from, to, 2);

        graph.insert_min(from, to, 1);

        assert_eq!(Some(&1), graph.get(&edge));
    }

    #[test]
    fn insert_max_if_vacant() {
        let mut alloc = Allocator::<FixedArena>::default();
        let mut graph = Graph::<FixedArena, u32>::default();

        let from = alloc.create();
        let to = alloc.create();
        let edge = Edge { from, to };

        graph.insert_max(from, to, 1);

        assert_eq!(Some(&1), graph.get(&edge));
    }

    #[test]
    fn insert_max_if_occupied_and_greater() {
        let mut alloc = Allocator::<FixedArena>::default();
        let mut graph = Graph::<FixedArena, u32>::default();

        let from = alloc.create();
        let to = alloc.create();
        let edge = Edge { from, to };

        graph.insert_ids(from, to, 0);

        graph.insert_max(from, to, 1);

        assert_eq!(Some(&1), graph.get(&edge));
    }

    #[test]
    fn insert_max_if_occupied_and_lesser() {
        let mut alloc = Allocator::<FixedArena>::default();
        let mut graph = Graph::<FixedArena, u32>::default();

        let from = alloc.create();
        let to = alloc.create();
        let edge = Edge { from, to };

        graph.insert_ids(from, to, 2);

        graph.insert_max(from, to, 1);

        assert_eq!(Some(&2), graph.get(&edge));
    }


}