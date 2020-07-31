use crate::*;
use std::fmt::{Result, Display, Formatter};

pub trait Arena {
    type Allocator;
}

#[macro_export]
macro_rules! fixed_arena {
    ($arena:ty) => {
        impl Arena for $arena {
            type Allocator = FixedAllocator<Self>;
        }
    };
}

#[macro_export]
macro_rules! dynamic_arena {
    ($arena:ty) => {
        impl Arena for $arena {
            type Allocator = DynamicAllocator<Self>;
        }
    };
}

pub trait DisplayEntity: Sized {
    fn fmt_entity<I: Indexes<Self>>(&self, id: I, f: &mut Formatter) -> Result;
}

pub struct Entity<'a, A, I> {
    pub arena: &'a A,
    pub id: I,
}

impl<A: Arena + DisplayEntity, I: Indexes<A> + Copy> Display for Entity<'_, A, I> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        self.arena.fmt_entity(self.id, f)
    }
}