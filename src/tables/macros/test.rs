use crate::*;

#[derive(Debug)]
pub struct Colony;

dynamic_arena!(Colony);

#[derive(Debug)]
pub struct Freighter;

fixed_arena!(Freighter);

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Time(f64);

table_array! {
    struct FreighterState {
        type Arena = Freighter;
        type RowEnum = enum FreighterStateRow;
        type IndexEnum = enum FreighterStateIndex;
        tables {
            idle: struct Idle {
                type Row = struct IdleRow;
                fields {
                    arrival: Time,
                }
                links {
                    location: Colony,
                }
            },
            moving: struct Moving {
                type Row = struct MovingRow;
                fields {

                }
                links {
                    from: Colony,
                    to: Colony,
                }
            },
        }
        transitions {}
    }
}

#[test]
fn test() {
    let mut a = Allocator::<Freighter>::default();
    let mut colonies = Allocator::<Colony>::default();
    let mut s = FreighterState::default();

    let c = colonies.create();
    let id = a.create();
    let row = IdleRow::new(id, Time(0.0), c);
    s.insert(id, row);

    // panic!("{:#?}", s);
}

#[derive(Debug, Default)]
pub struct ArenaA;
dynamic_arena!(ArenaA);

#[derive(Debug, Default)]
pub struct ArenaB;
dynamic_arena!(ArenaB);

#[derive(Debug, Default)]
pub struct ArenaC;
dynamic_arena!(ArenaC);

#[derive(Debug, Default)]
pub struct TableC {
    pub ids: IdColumn<Self, ArenaC>,
    pub value: Column<Self, u32>,
    pub id_a: IdColumn<Self, ArenaA>,
    pub id_b: IdColumn<Self, ArenaB>,
}

impl TableC {
    pub fn push<A: ValidId<ArenaA>, B: ValidId<ArenaB>, ID: ValidId<ArenaC>>(
        &mut self,
        row: RowC<ID, A, B>,
    ) -> Index<Self> {
        self.value.push(row.value);
        self.id_a.push(Some(row.id_a));
        self.id_b.push(Some(row.id_b));
        self.ids.push(Some(row.id))
    }

    fn swap_remove<'a>(
        &mut self,
        index: &Index<Self>,
        allocator_a: &'a Allocator<ArenaA>,
        allocator_b: &'a Allocator<ArenaB>,
    ) -> Option<RowC<Valid<'a, Id<ArenaC>>, Valid<'a, Id<ArenaA>>, Valid<'a, Id<ArenaB>>>> {
        let id = Valid::assert(self.ids.swap_remove(index).unwrap());
        let value = self.value.swap_remove(index);

        let id_a = self
            .id_a
            .swap_remove(index)
            .and_then(|id| allocator_a.validate(id));

        let id_b = self
            .id_b
            .swap_remove(index)
            .and_then(|id| allocator_b.validate(id));

        Some(RowC {
            id,
            value,
            id_a: id_a?,
            id_b: id_b?,
        })
    }
}

pub struct RowC<ID, A, B> {
    id: ID,
    value: u32,
    id_a: A,
    id_b: B,
}
