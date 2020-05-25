use std::fmt::Debug;
use std::hash::Hash;

pub trait Arena: Debug + Default {
    type Index: crate::Index;
    type Generation: Debug + Copy + Eq + Hash;
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
