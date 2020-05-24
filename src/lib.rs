use std::convert::TryInto;
use std::fmt::Debug;
use std::hash::Hash;
use paste::item;

pub use index::*;
pub use generation::*;
pub use ids::*;
pub use allocator::*;
pub use component::*;
pub use map::*;

mod index;
mod generation;
mod ids;
mod allocator;
mod component;
mod map;

pub trait Arena: Debug + Default {
    type Index: Index;
    type NextIndex: Default + Debug;
    type Generation: Copy + Eq + Hash + Fixed;
    type Generations: Default + Debug;
    type Dead: Default + Debug;
}

#[macro_export]
macro_rules! fixed_arena {
    ($arena:ty, $index:ty) => {
        impl Arena for $arena {
            type Index = $index;
            type NextIndex = Self::Index;
            type Generation = ();
            type Generations = ();
            type Dead = ();
        }
    }
}

#[macro_export]
macro_rules! dynamic_arena {
    ($arena:ty, $index:ty, $gen:ty) => {
        impl Arena for $arena {
            type Index = $index;
            type NextIndex = ();
            type Generation = $gen;
            type Generations = Vec<Self::Generation>;
            type Dead = Vec<Self::Index>;
        }
    }
}
