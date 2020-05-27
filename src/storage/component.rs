use crate::*;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct Component<A: Arena, T> {
    values: Vec<T>,
    marker: PhantomData<A>,
}

impl<A: Arena, T> Default for Component<A, T> {
    fn default() -> Self {
        Self {
            values: vec![],
            marker: PhantomData,
        }
    }
}

impl<A: Arena, T, I: Indexes<A>> Get<I, T> for Component<A, T> {
    fn get(&self, id: I) -> &T {
        self.get_index(id.index())
    }

    fn get_mut(&mut self, id: I) -> &mut T {
        self.get_index_mut(id.index())
    }
}

impl<A: Arena, T, I: Indexes<A>> Insert<I, T> for Component<A, T> {
    fn insert(&mut self, id: I, value: T) {
        self.insert_index(id.index(), value);
    }
}

impl<A: Arena, T> Component<A, T> {
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.values.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.values.iter_mut()
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    fn get_index(&self, id: usize) -> &T {
        &self.values[id]
    }

    fn get_index_mut(&mut self, id: usize) -> &mut T {
        &mut self.values[id]
    }

    fn insert_index(&mut self, index: usize, value: T) {
        if let Some(component) = self.values.get_mut(index) {
            *component = value;
        } else if self.values.len() == index {
            self.values.push(value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::FixedArena;

    #[test]
    #[should_panic]
    fn get_at_invalid_index_panics() {
        let components = Component::<FixedArena, u32>::default();
        let id = Id { index: 0, gen: () };

        components.get(id);
    }

    #[test]
    #[should_panic]
    fn get_mut_at_invalid_index_panics() {
        let mut components = Component::<FixedArena, u32>::default();
        let id = Id { index: 0, gen: () };

        components.get_mut(id);
    }

    #[test]
    fn get_at_valid_index() {
        let mut components = Component::<FixedArena, u32>::default();
        let id = Id { index: 0, gen: () };

        components.insert(id, 5);

        assert_eq!(&5, components.get(id));
    }

    #[test]
    fn insert_at_index_equals_len_extends_vec() {
        let mut components = Component::<FixedArena, u32>::default();
        let id = Id { index: 0, gen: () };

        components.insert(id, 5);

        assert_eq!(1, components.len());
        assert_eq!(5, components.values[0]);
    }

    #[test]
    fn insert_at_index_less_than_len_replaces_existing_value() {
        let mut components = Component::<FixedArena, u32>::default();
        let id = Id { index: 0, gen: () };

        components.insert(id, 3);
        components.insert(id, 5);

        assert_eq!(1, components.len());
        assert_eq!(5, components.values[0]);
    }
}