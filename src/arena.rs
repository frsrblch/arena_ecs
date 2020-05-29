use crate::*;
use std::fmt::Debug;
use std::hash::Hash;
use std::fmt::{Result, Display, Formatter};

pub trait Arena: Debug + Default {
    type Index: crate::Index;
    type Generation: Debug + Copy + Eq + Hash;
    type Allocator: Debug + Default;

    fn entity<I: Indexes<Self>>(&self, id: I) -> Entity<Self, I> {
        Entity { arena: self, id }
    }
}

#[macro_export]
macro_rules! fixed_arena {
    ($arena:ty, $index:ty) => {
        impl Arena for $arena {
            type Index = $index;
            type Generation = ();
            type Allocator = FixedAllocator<Self>;
        }
    };
}

#[macro_export]
macro_rules! dynamic_arena {
    ($arena:ty, $index:ty, $gen:ty) => {
        impl Arena for $arena {
            type Index = $index;
            type Generation = $gen;
            type Allocator = DynamicAllocator<Self>;
        }
    };
}

pub trait DisplayEntity: Arena {
    fn fmt_entity<I: Indexes<Self>>(&self, id: I, f: &mut Formatter<'_>) -> Result;
}

pub struct Entity<'a, A: Arena, I: Indexes<A>> {
    pub arena: &'a A,
    pub id: I,
}

impl<A: Arena + DisplayEntity, I: Indexes<A>> Display for Entity<'_, A, I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.arena.fmt_entity(self.id, f)
    }
}