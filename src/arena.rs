use std::fmt::Debug;
use crate::{Index, Fixed};
use std::hash::Hash;

pub trait Arena: Debug + Default {
    type Index: Index;
    type Generation: Copy + Eq + Hash + Fixed;
    type Allocator: Debug + Default;
}

#[macro_export]
macro_rules! fixed_arena {
($arena:ty, $index:ty) => {
    impl Arena for $arena {
        type Index = $index;
        type Generation = ();
        type Allocator = FixedAllocator<Self>;
    }
}
}

#[macro_export]
macro_rules! dynamic_arena {
($arena:ty, $index:ty, $gen:ty) => {
    impl Arena for $arena {
        type Index = $index;
        type Generation = $gen;
        type Allocator = DynamicAllocator<Self>;
    }
}
}
