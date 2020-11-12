use crate::GenerationCmp;
use crate::*;

#[derive(Debug)]
pub struct IdColumn<C, ID> {
    ids: Column<C, Option<Id<ID>>>,
    generation: AllocGen<ID>,
}

impl<C, ID> Default for IdColumn<C, ID> {
    fn default() -> Self {
        Self {
            ids: Column::default(),
            generation: AllocGen::default(),
        }
    }
}

impl<C, ID> IdColumn<C, ID> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            ids: Column::with_capacity(capacity),
            generation: AllocGen::default(),
        }
    }

    pub fn push<I: ValidId<ID>>(&mut self, id: Option<I>) -> Index<C> {
        self.ids.push(id.map(|id| id.id()))
    }

    pub fn swap_remove(&mut self, index: &Index<C>) -> Option<Id<ID>> {
        self.ids.swap_remove(index)
    }

    pub fn get(&self, index: &Index<C>) -> Option<&Id<ID>> {
        self.ids.get(index).map(|opt| opt.as_ref()).flatten()
    }

    pub fn get_mut(&mut self, index: &Index<C>) -> Option<&mut Id<ID>> {
        self.ids.get_mut(index).map(|opt| opt.as_mut()).flatten()
    }

    pub fn len(&self) -> usize {
        self.ids.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ids.is_empty()
    }

    pub fn iter(&self) -> Iter<C, Option<Id<ID>>> {
        self.ids.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<C, Option<Id<ID>>> {
        self.ids.iter_mut()
    }

    pub fn indices(&self) -> super::column::Indices<C> {
        self.ids.indices()
    }
}

impl<C, ID: Arena<Allocator = DynamicAllocator<ID>>> IdColumn<C, ID> {
    pub fn validate<'a>(&'a mut self, allocator: &'a Allocator<ID>) -> Valid<'a, &Self> {
        self.synchronize(allocator);

        Valid::new(self)
    }

    fn synchronize(&mut self, allocator: &Allocator<ID>) {
        match allocator.generation_cmp(self.generation) {
            GenerationCmp::Valid => {}
            GenerationCmp::OffByOne(killed) => {
                for opt_id in self.iter_mut() {
                    if *opt_id == Some(killed) {
                        *opt_id = None;
                    }
                }
                self.generation = allocator.generation();
            }
            GenerationCmp::Outdated => {
                for opt_id in self.iter_mut() {
                    if let Some(id) = opt_id {
                        if !id.is_alive(allocator) {
                            *opt_id = None;
                        }
                    }
                }
                self.generation = allocator.generation();
            }
        }
    }

    pub fn kill(&mut self, id: Id<ID>) {
        for opt_id in self.iter_mut() {
            if *opt_id == Some(id) {
                *opt_id = None;
            }
        }

        self.generation.increment();
    }
}

impl<'a, C, ID> Valid<'a, &'a IdColumn<C, ID>> {
    pub fn iter(&self) -> Valid<'a, Iter<C, Option<Id<ID>>>> {
        Valid::new(self.value.iter())
    }
}
