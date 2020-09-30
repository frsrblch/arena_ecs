use crate::*;
use std::cmp::*;
use fnv::FnvHashMap as HashMap;

#[derive(Debug)]
pub struct Edge<A, W> {
    weight: W,
    to: Id<A>,
}

impl<A, W: Clone> Clone for Edge<A, W> {
    fn clone(&self) -> Self {
        Self {
            weight: self.weight.clone(),
            to: self.to,
        }
    }
}

impl<A, W: Copy> Copy for Edge<A, W> {}

impl<A, W: PartialEq> PartialEq for Edge<A, W> {
    fn eq(&self, other: &Self) -> bool {
        self.weight.eq(&other.weight) && self.to.eq(&other.to)
    }
}

impl<A, W: PartialOrd> PartialOrd for Edge<A, W> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}

#[derive(Debug, Default)]
pub struct Graph<A, W> {
    edges: HashMap<Pair<A>, W>,
    generation: u64,
}

impl<A, W> Graph<A, W> {
    pub fn insert<I: Indexes<A>>(&mut self, a: I, b: I, weight: W) {
        let key = Pair::new(a.id(), b.id());
        self.edges.insert(key, weight);
    }

    pub fn remove(&mut self, a: Id<A>, b: Id<A>) -> Option<W> {
        let key = Pair::new(a, b);
        self.edges.remove(&key)
    }

    pub fn kill(&mut self, id: Id<A>) {
        self.edges.retain(|pair, _| !pair.contains(id));

        self.generation += 1;
    }

    pub fn clear(&mut self) {
        self.edges.clear();
    }

    pub fn get_edges<I: Indexes<A>>(&self, node: I) -> impl Iterator<Item=(&Pair<A>, &W)> + '_ {
        let node = node.id();
        self.edges
            .iter()
            .filter(move |&(e, _)| e.contains(node))
    }
}

impl<A: Arena<Allocator=DynamicAllocator<A>>, W> Graph<A, W> {
    pub fn retain_living(&mut self, allocator: &Allocator<A>) {
        self.edges.retain(|k, _| allocator.is_alive(k.a()) && allocator.is_alive(k.b()));
    }

    // TODO rework using Allocator generation
    pub fn get_valid_edges<'a, I: Indexes<A> + 'a>(
        &'a mut self,
        id: I,
        allocator: &'a Allocator<A>
    ) -> impl Iterator<Item=((Valid<'a, A>, Valid<'a, A>), &W)> + 'a {
        self.get_edges(id)
            .filter_map(move |(pair, w)| {
                pair.validate(allocator).map(|p| (p, w))
            })
    }
}