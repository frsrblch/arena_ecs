pub use component::*;
pub use graph::*;
pub use id_link::IdLink;
pub use map::*;

mod component;
mod graph;
mod id_link;
mod map;

mod testing {
    use std::iter::FlatMap;
    use std::slice::{Iter, IterMut};

    pub struct Values {
        array: Vec<[f64; 8]>,
    }

    impl Values {
        pub fn get(&self, index: usize) -> Option<&f64> {
            let outer = index / 8;
            let inner = index % 8;

            self.array.get(outer).and_then(|array| array.get(inner))
        }

        pub fn iter(&self) -> FlatMap<Iter<[f64; 8]>, Iter<f64>, ArrayToValue> {
            self.array.iter().flat_map(|a| a.iter())
        }

        pub fn iter_mut(&mut self) -> FlatMap<IterMut<[f64; 8]>, IterMut<f64>, ArrayToValueMut> {
            self.array.iter_mut().flat_map(|a| a.iter_mut())
        }

        pub fn get_mut(&mut self, index: usize) -> Option<&mut f64> {
            let outer = index / 8;
            let inner = index % 8;

            self.array
                .get_mut(outer)
                .and_then(|array| array.get_mut(inner))
        }

        pub fn insert(&mut self, index: usize, value: f64) {
            let outer = index / 8;

            if self.array.len() > outer {
                let n = self.array.len() - outer;

                let extension = std::iter::repeat_with(|| [0.0; 8]).take(n);

                self.array.extend(extension);
            }

            if let Some(v) = self.get_mut(index) {
                *v = value;
            }
        }
    }

    use std::ops::*;
    impl AddAssign<&Self> for Values {
        fn add_assign(&mut self, rhs: &Values) {
            self.array
                .iter_mut()
                .zip(rhs.array.iter())
                .for_each(|(a, b)| {
                    a.iter_mut().zip(b.iter()).for_each(|(a, b)| {
                        *a += *b;
                    });
                });
        }
    }

    type ArrayToValue = fn(&[f64; 8]) -> Iter<f64>;
    type ArrayToValueMut = fn(&mut [f64; 8]) -> IterMut<f64>;

    impl<'a> IntoIterator for &'a Values {
        type Item = &'a f64;
        type IntoIter = FlatMap<Iter<'a, [f64; 8]>, Iter<'a, f64>, ArrayToValue>;

        fn into_iter(self) -> Self::IntoIter {
            self.array.iter().flat_map(|a| a.iter())
        }
    }

    impl<'a> IntoIterator for &'a mut Values {
        type Item = &'a mut f64;
        type IntoIter = FlatMap<IterMut<'a, [f64; 8]>, IterMut<'a, f64>, ArrayToValueMut>;

        fn into_iter(self) -> Self::IntoIter {
            self.array.iter_mut().flat_map(|a| a.iter_mut())
        }
    }

    #[test]
    fn size() {
        assert_eq!(8, std::mem::size_of::<f64>());
    }
}
