use paste::item;

pub use allocator::*;
pub use arena::*;
pub use ids::*;
pub use storage::*;

mod allocator;
mod arena;
mod ids;
mod storage;

pub trait Create<ROW> {
    type Id;
    fn create(&mut self, row: ROW) -> Self::Id;
}

pub trait CreateLinked<ROW> {
    type Links;
    type Id;
    fn create_linked(&mut self, row: ROW, links: Self::Links) -> Self::Id;
}
