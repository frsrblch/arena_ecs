use std::convert::TryInto;
use std::fmt::Debug;
use std::hash::Hash;
use paste::item;

pub use ids::*;
pub use allocator::*;
pub use component::*;
pub use map::*;

mod ids;
mod allocator;
mod component;
mod map;

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
