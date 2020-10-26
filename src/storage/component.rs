use crate::*;
use std::marker::PhantomData;
use std::ops::AddAssign;

// #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub struct Component<ID, T> {
    values: Vec<T>,
    marker: PhantomData<ID>,
}

impl<ID, T> Default for Component<ID, T> {
    fn default() -> Self {
        Self {
            values: vec![],
            marker: PhantomData,
        }
    }
}

impl<ID, T> Component<ID, T> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            values: Vec::with_capacity(capacity),
            marker: PhantomData,
        }
    }

    pub fn iter(&self) -> Iter<ID, T> {
        Iter::new(self.values.iter())
    }

    pub fn iter_mut(&mut self) -> IterMut<ID, T> {
        IterMut::new(self.values.iter_mut())
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get<I: ValidId<ID>>(&self, id: I) -> &T {
        assert!(
            id.index() < self.values.len(),
            format!("Invalid index: {:?}", std::any::type_name::<Self>())
        );

        self.values.get(id.index()).unwrap()
    }

    pub fn get_mut<I: ValidId<ID>>(&mut self, id: I) -> &mut T {
        assert!(
            id.index() < self.values.len(),
            format!("Invalid index: {:?}", std::any::type_name::<Self>())
        );

        self.values.get_mut(id.index()).unwrap()
    }

    pub fn insert<I: ValidId<ID>>(&mut self, id: I, value: T) {
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

impl<ID1, T> Component<ID1, T>
where
    ID1: Arena<Allocator = DynamicAllocator<ID1>>,
    T: AddAssign<T> + Copy + Default,
{
    pub fn sum_from<ID2: Arena>(
        &mut self,
        component: &Component<ID2, T>,
        link: &Component<ID2, Id<ID1>>,
        alloc: &Allocator<ID1>,
    ) {
        self.fill_with(Default::default);

        component.zip(link).for_each(|(value, id)| {
            if let Some(id) = alloc.validate(*id) {
                let self_value = self.get_mut(id);
                *self_value += *value;
            }
        });
    }

    pub fn sum_from_link<ID2: Arena>(
        &mut self,
        component: &Component<ID2, T>,
        link: &mut IdLink<ID2, ID1>,
        alloc: &Allocator<ID1>,
    ) {
        self.fill_with(Default::default);

        let link = link.validate(alloc);

        for (value, id) in component.iter().zip(&link) {
            if let Some(id) = id {
                *self.get_mut(id) += *value;
            }
        }
    }
}

impl<'a, ID, T> IntoIterator for &'a Component<ID, T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.iter()
    }
}

impl<'a, ID, T> IntoIterator for &'a mut Component<ID, T> {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.iter_mut()
    }
}

impl<'a, ID, T> TypedIterator for &'a Component<ID, T> {
    type Context = ID;
}

impl<'a, ID, T> TypedIterator for &'a mut Component<ID, T> {
    type Context = ID;
}

#[cfg(feature = "rayon")]
impl<ID, T: Send + Sync> Component<ID, T> {
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
