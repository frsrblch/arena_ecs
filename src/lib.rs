use paste::item;

pub use allocator::*;
pub use arena::*;
pub use ids::*;
pub use storage::*;
pub use traits::*;

mod allocator;
mod arena;
mod ids;
mod storage;
mod traits;

#[macro_export]
macro_rules! unwrap_return {
    ($e:expr) => {
        if let Some(value) = $e {
            value
        } else {
            return;
        }
    }
}