fn main() {

}

use ecs_traits::*;

#[derive(Debug, Default)]
pub struct System;

impl Arena for System {
    type Index = u16;
    type Generation = ();
    type Dead = ();
}

#[derive(Debug, Default)]
pub struct Colony;

impl Arena for Colony {
    type Index = u16;
    type Generation = NonZeroU16;
}

#[test]
fn id_size() {
    assert_eq!(2, std::mem::size_of::<Id<System>>());
    assert_eq!(4, std::mem::size_of::<Option<Id<System>>>());

    assert_eq!(4, std::mem::size_of::<Id<Colony>>());
    assert_eq!(4, std::mem::size_of::<Option<Id<Colony>>>());
}