use crate::*;
use std::marker::PhantomData;
use std::slice::{Iter, IterMut};

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

impl<A, T: std::fmt::Debug> Component<A, T> {
    pub fn iter(&self) -> Iter<T> {
        self.values.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        self.values.iter_mut()
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn get<I: Indexes<A>>(&self, id: I) -> &T {
        assert!(
            id.index() < self.values.len(),
            format!("Invalid index: {:?}", std::any::type_name::<Self>())
        );

        self.values.get(id.index()).unwrap()
    }

    pub fn get_mut<I: Indexes<A>>(&mut self, id: I) -> &mut T {
        assert!(
            id.index() < self.values.len(),
            format!("Invalid index: {:?}", std::any::type_name::<Self>())
        );

        self.values.get_mut(id.index()).unwrap()
    }

    pub fn insert<I: Indexes<A>>(&mut self, id: I, value: T) {
        if let Some(component) = self.values.get_mut(id.index()) {
            *component = value;
        } else if self.len() == id.index() {
            self.values.push(value);
        } else {
            panic!("Invalid index: {:?}", std::any::type_name::<Self>());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::FixedArena;

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