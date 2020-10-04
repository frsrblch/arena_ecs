use crate::*;
use std::marker::PhantomData;
use std::slice::{Iter, IterMut};

#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub struct Component<A, T> {
    values: Vec<T>,
    marker: PhantomData<A>,
}

impl<A, T> Default for Component<A, T> {
    fn default() -> Self {
        Self {
            values: vec![],
            marker: PhantomData,
        }
    }
}

impl<A, T> Component<A, T> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            values: Vec::with_capacity(capacity),
            marker: PhantomData,
        }
    }

    pub fn iter(&self) -> Iter<T> {
        self.values.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        self.values.iter_mut()
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get<I: ValidId<A>>(&self, id: I) -> &T {
        assert!(
            id.index() < self.values.len(),
            format!("Invalid index: {:?}", std::any::type_name::<Self>())
        );

        self.values.get(id.index()).unwrap()
    }

    pub fn get_mut<I: ValidId<A>>(&mut self, id: I) -> &mut T {
        assert!(
            id.index() < self.values.len(),
            format!("Invalid index: {:?}", std::any::type_name::<Self>())
        );

        self.values.get_mut(id.index()).unwrap()
    }

    pub fn insert<I: ValidId<A>>(&mut self, id: I, value: T) {
        if let Some(component) = self.values.get_mut(id.index()) {
            *component = value;
        } else if self.len() == id.index() {
            self.values.push(value);
        } else {
            panic!("Invalid index: {:?}", std::any::type_name::<Self>());
        }
    }

    pub fn fill_with<F: FnMut() -> T>(&mut self, mut f: F) {
        self.iter_mut().for_each(|v| *v = f());
    }
}

impl<ID, T: Clone> Component<ID, T> {
    pub fn fill(&mut self, value: T) {
        self.iter_mut().for_each(|v| *v = value.clone());
    }
}

impl<ID1: Arena<Allocator=DynamicAllocator<ID1>>, T: std::ops::AddAssign<T> + Copy + Default> Component<ID1, T> {
    pub fn sum_from<ID2>(&mut self, component: &Component<ID2, T>, link: &Component<ID2, Id<ID1>>, alloc: &Allocator<ID1>) {
        self.fill_with(Default::default);

        component.iter()
            .zip(link.iter())
            .for_each(|(component_value, id)| {
                if let Some(id) = alloc.validate(*id) {
                    let value = self.get_mut(id);
                    *value += *component_value;
                }
            });
    }

    pub fn sum_from_opt<ID2>(&mut self, component: &Component<ID2, T>, link: &Component<ID2, Option<Id<ID1>>>, alloc: &Allocator<ID1>) {
        self.fill_with(Default::default);

        component.iter()
            .zip(link.iter())
            .for_each(|(component_value, id)| {
                if let Some(id) = id {
                    if let Some(id) = alloc.validate(*id) {
                        let value = self.get_mut(id);
                        *value += *component_value;
                    }
                }
            });
    }
}

#[cfg(feature = "rayon")]
impl<A, T: Send + Sync> Component<A, T> {
    pub fn par_iter(&self) -> rayon::slice::Iter<T> {
        self.values.par_iter()
    }

    pub fn par_iter_mut(&mut self) -> rayon::slice::IterMut<T> {
        self.values.par_iter_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::allocator::test::FixedArena;

    #[test]
    fn get_at_valid_index() {
        let mut components = Component::<FixedArena, u32>::default();
        let id = Id::first(0);

        components.insert(id, 5);

        assert_eq!(&5, components.get(id));
    }

    #[test]
    fn insert_at_index_equals_len_extends_vec() {
        let mut components = Component::<FixedArena, u32>::default();
        let id = Id::first(0);

        components.insert(id, 5);

        assert_eq!(1, components.len());
        assert_eq!(5, components.values[0]);
    }

    #[test]
    fn insert_at_index_less_than_len_replaces_existing_value() {
        let mut components = Component::<FixedArena, u32>::default();
        let id = Id::first(0);

        components.insert(id, 3);
        components.insert(id, 5);

        assert_eq!(1, components.len());
        assert_eq!(5, components.values[0]);
    }

    #[test]
    #[should_panic]
    fn get_given_invalid_id_panics() {
        let id = Id::first(0);
        let component = Component::<FixedArena, u32>::default();

        component.get(id);
    }

    #[test]
    #[should_panic]
    fn get_mut_given_invalid_id_panics() {
        let id = Id::first(0);
        let mut component = Component::<FixedArena, u32>::default();

        component.get_mut(id);
    }

    #[test]
    #[should_panic]
    fn insert_given_invalid_id_panics() {
        let id = Id::first(1);
        let mut component = Component::<FixedArena, u32>::default();

        component.insert(id, 0);
    }
}