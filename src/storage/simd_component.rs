use packed_simd::*;
use std::marker::PhantomData;
use crate::*;

pub trait Vector: Copy + Sized {
    type Type: Base<Vector=Self>;
    fn extract(self, index: usize) -> Self::Type;
    fn replace(self, index: usize, value: Self::Type) -> Self;
    fn splat(value: Self::Type) -> Self;
    const LANES: usize;
}

pub trait Base: Sized {
    type Vector: Vector<Type=Self>;
}

impl Vector for f32x8 {
    type Type = f32;

    fn extract(self, index: usize) -> Self::Type {
        self.extract(index)
    }

    fn replace(self, index: usize, value: Self::Type) -> Self {
        self.replace(index, value)
    }

    fn splat(value: Self::Type) -> Self {
        Self::splat(value)
    }

    const LANES: usize = Self::lanes();
}

impl Vector for f64x4 {
    type Type = f64;

    fn extract(self, index: usize) -> Self::Type {
        self.extract(index)
    }

    fn replace(self, index: usize, value: Self::Type) -> Self {
        self.replace(index, value)
    }

    fn splat(value: Self::Type) -> Self {
        Self::splat(value)
    }

    const LANES: usize = Self::lanes();
}

impl Vector for u8x32 {
    type Type = u8;

    fn extract(self, index: usize) -> Self::Type {
        self.extract(index)
    }

    fn replace(self, index: usize, value: Self::Type) -> Self {
        self.replace(index, value)
    }

    fn splat(value: Self::Type) -> Self {
        Self::splat(value)
    }

    const LANES: usize = Self::lanes();
}

impl Vector for u16x16 {
    type Type = u16;

    fn extract(self, index: usize) -> Self::Type {
        self.extract(index)
    }

    fn replace(self, index: usize, value: Self::Type) -> Self {
        self.replace(index, value)
    }

    fn splat(value: Self::Type) -> Self {
        Self::splat(value)
    }

    const LANES: usize = Self::lanes();
}

impl Vector for u32x8 {
    type Type = u32;

    fn extract(self, index: usize) -> Self::Type {
        self.extract(index)
    }

    fn replace(self, index: usize, value: Self::Type) -> Self {
        self.replace(index, value)
    }

    fn splat(value: Self::Type) -> Self {
        Self::splat(value)
    }

    const LANES: usize = Self::lanes();
}

impl Vector for u64x4 {
    type Type = u64;

    fn extract(self, index: usize) -> Self::Type {
        self.extract(index)
    }

    fn replace(self, index: usize, value: Self::Type) -> Self {
        self.replace(index, value)
    }

    fn splat(value: Self::Type) -> Self {
        Self::splat(value)
    }

    const LANES: usize = Self::lanes();
}

impl Base for f32 {
    type Vector = f32x8;
}

impl Base for f64 {
    type Vector = f64x4;
}

impl Base for u8 {
    type Vector = u8x32;
}

impl Base for u16 {
    type Vector = u16x16;
}

impl Base for u32 {
    type Vector = u32x8;
}

impl Base for u64 {
    type Vector = u64x4;
}

pub trait Comp: Copy + Default {
    type Base: Base;
    fn to_base(self) -> Self::Base;
    fn from_base(value: Self::Base) -> Self;
    fn default_vector() -> <Self::Base as Base>::Vector {
        <Self::Base as Base>::Vector::splat(Self::default().to_base())
    }
}

impl Comp for f32 {
    type Base = f32;

    fn to_base(self) -> f32 {
        self
    }

    fn from_base(value: f32) -> Self {
        value
    }
}

impl Comp for f64 {
    type Base = f64;

    fn to_base(self) -> f64 {
        self
    }

    fn from_base(value: f64) -> Self {
        value
    }
}

pub struct SimdComponent<A: Arena, T: Comp> {
    values: Vec<<<T as Comp>::Base as Base>::Vector>,
    len: usize,
    marker: PhantomData<A>,
}

impl<A: Arena, T: Comp> Default for SimdComponent<A, T> {
    fn default() -> Self {
        Self {
            values: vec![],
            len: 0,
            marker: PhantomData,
        }
    }
}

impl<A: Arena, T: Comp> SimdComponent<A, T> {
    pub fn insert_index(&mut self, index: usize, value: T) {
        let vector_index = index / Self::LANES;
        let lane = index % Self::LANES;

        if vector_index >= self.values.len() {
            let new_vector_length = vector_index - self.values.len() + 1;

            let new_vectors = std::iter::repeat_with(T::default_vector)
                .take(new_vector_length);

            self.values.extend(new_vectors);
        }

        self.len = self.len.max(index + 1);

        if let Some(vector) = self.values.get_mut(vector_index) {
            *vector = vector.replace(lane, value.to_base());
        }
    }

    pub fn get_index(&self, index: usize) -> Option<T> {
        if index > self.len {
            return None;
        }

        let vector_index = index / Self::LANES;
        let lane = index % Self::LANES;

        self.values
            .get(vector_index)
            .map(|vector| T::from_base(vector.extract(lane)))
    }

    pub fn len(&self) -> usize {
        self.len
    }

    const LANES: usize = <<<T as Comp>::Base as Base>::Vector as Vector>::LANES;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::allocator::test::FixedArena;

    #[derive(Debug, Default, Copy, Clone, PartialEq)]
    struct TestComponent(f32);

    impl Comp for TestComponent {
        type Base = f32;

        fn to_base(self) -> Self::Base {
            self.0
        }

        fn from_base(value: Self::Base) -> Self {
            Self(value)
        }
    }

    #[test]
    fn insert_at_0() {
        let mut s = SimdComponent::<FixedArena, TestComponent>::default();

        s.insert_index(0, TestComponent(2.0));

        assert_eq!(1, s.len());
        assert_eq!(1, s.values.len());
        assert_eq!(2.0, s.values[0].extract(0));
    }

    #[test]
    fn insert_at_7() {
        let mut s = SimdComponent::<FixedArena, TestComponent>::default();

        s.insert_index(7, TestComponent(3.0));
        s.insert_index(0, TestComponent(2.0));

        assert_eq!(1, s.values.len());
        assert_eq!(8, s.len());
        assert_eq!(Some(TestComponent(2.0)), s.get_index(0));
        assert_eq!(Some(TestComponent(3.0)), s.get_index(7));
    }

    #[test]
    fn get_index() {
        let mut s = SimdComponent::<FixedArena, f32>::default();

        s.insert_index(7, 3.0);

        assert_eq!(Some(3.0), s.get_index(7));
    }

    #[test]
    fn insert_beyond_first_vector() {
        let mut s = SimdComponent::<FixedArena, f32>::default();

        s.insert_index(21, 3.0);

        assert_eq!(22, s.len());
        assert_eq!(Some(3.0), s.get_index(21));
    }
}