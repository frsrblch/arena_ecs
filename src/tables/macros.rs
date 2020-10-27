#[macro_export]
macro_rules! table_array {
    (
        struct $table:ident {
            type Arena = $arena:ident;
            type RowEnum = $row_enum:ident;
            type IndexEnum = $index_enum:ident;
            tables {
                $(
                    $field:ident: $variant:ident {
                        type Row = $v_row:ident;
                        $(
                            $v_field:ident: $v_t:ty,
                        )*
                    },
                )*
            }
            transitions {
                $( $t_field:ident: $t_variant:ident, )*
            }
        }
    ) => {
        #[derive(Debug, Default)]
        pub struct $table {
            indices: $crate::IdIndices<$arena, $index_enum>,
            $(
                pub $field: $variant,
            )*
            $(
                $t_field: $t_variant,
            )*
        }

        #[allow(dead_code)]
        impl $table {
            pub fn insert_row<I: $crate::ValidId<$arena>, R: Into<$row_enum>>(
                &mut self,
                id: I,
                row: R,
            ) {
                self.insert_row_inner(id, row.into());
            }

            fn insert_row_inner<I: $crate::ValidId<$arena>>(&mut self, id: I, row: $row_enum) {
                self.remove(id);
                let indices = &mut self.indices;
                match row {
                    $(
                        $row_enum::$variant(row) => self.$field.insert(id, row, indices),
                    )*
                };
            }

            pub fn remove<I: $crate::ValidId<$arena>>(&mut self, id: I) -> Option<$row_enum> {
                self.indices
                    .remove(id)
                    .map(|index| self.remove_index(index))
            }

            fn remove_index(&mut self, index: $index_enum) -> $row_enum {
                let indices = &mut self.indices;
                match index {
                    $(
                        $index_enum::$variant(index) => self.$field.swap_remove(index, indices).into(),
                    )*
                }
            }
        }

        row_enum! { enum $row_enum { $( $variant($v_row), )* } }
        index_enum! { enum $index_enum { $( $variant, )* } }

        $(
            table! {
                struct $variant {
                    type Arena = $arena;
                    type Row = $v_row;
                    type Index = $index_enum;
                    $(
                        $v_field: $v_t,
                    )*
                }
            }
        )*
    };
}

#[macro_export]
macro_rules! index_enum {
    (
        enum $name:ident {
            $( $variant:ident, )*
        }
    ) => {
        #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
        pub enum $name {
            $(
                $variant($crate::Index<$variant>),
            )*
        }

        $(
            impl From<$crate::Index<$variant>> for $name {
                fn from(value: $crate::Index<$variant>) -> Self {
                    Self::$variant(value)
                }
            }
        )*
    }
}

#[macro_export]
macro_rules! row_enum {
    (
        enum $name:ident {
            $( $variant:ident($v_row:ident), )*
        }
    ) => {
        #[derive(Debug)]
        pub enum $name {
            $(
                $variant($v_row),
            )*
        }

        $(
            impl From<$v_row> for $name {
                fn from(value: $v_row) -> Self {
                    Self::$variant(value)
                }
            }
        )*
    }
}

#[macro_export]
macro_rules! table {
    (
        struct $table:ident {
            type Arena = $arena:ident;
            type Row = $row:ident;
            type Index = $state_index:ident;
            $( $field:ident: $t:ty, )* $(,)?
        }
    ) => {
        #[derive(Debug)]
        pub struct $table {
            pub id: $crate::Column<Self, $crate::Id<$arena>>,
            $(
                pub $field: $crate::Column<Self, $t>,
            )*
        }

        impl Default for $table {
            fn default() -> Self {
                Self {
                    id: Default::default(),
                    $(
                        $field: Default::default(),
                    )*
                }
            }
        }

        #[allow(dead_code)]
        impl $table {
            pub fn insert<I: $crate::ValidId<$arena>>(
                &mut self,
                id: I,
                row: $row,
                indices: &mut $crate::IdIndices<$arena, $state_index>
            ) -> $crate::Index<Self> {
                let index = self.insert_inner(row);
                indices.insert(id, index);
                index
            }

            fn insert_inner(&mut self, row: $row) -> $crate::Index<Self> {
                $(
                    self.$field.push(row.$field);
                )*
                self.id.push(row.id)
            }

            pub fn swap_remove(
                &mut self,
                index: $crate::Index<Self>,
                indices: &mut $crate::IdIndices<$arena, $state_index>
            ) -> $row {
                let row = self.swap_remove_inner(index);
                if let Some(swapped) = self.id.get(index).map($crate::Valid::assert) {
                    indices.insert(swapped, index);
                }
                row
            }

            fn swap_remove_inner(&mut self, index: $crate::Index<Self>) -> $row {
                $row {
                    id: self.id.swap_remove(index),
                    $(
                        $field: self.$field.swap_remove(index),
                    )*
                }
            }

            pub fn indices(&self) -> $crate::Indices<Self> {
                self.id.indices()
            }

            pub fn len(&self) -> usize {
                self.id.len()
            }
        }

        #[derive(Debug)]
        pub struct $row {
            pub id: $crate::Id<$arena>,
            $(
                pub $field: $t,
            )*
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[derive(Debug)]
    pub struct Colony;

    fixed_arena!(Colony);

    #[derive(Debug)]
    pub struct Freighter;

    fixed_arena!(Freighter);

    #[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
    pub struct Time(f64);

    table_array! {
        struct FreighterState {
            type Arena = Freighter;
            type RowEnum = FreighterStateRow;
            type IndexEnum = FreighterStateIndex;
            tables {
                idle: Idle {
                    type Row = IdleRow;
                    location: Id<Colony>,
                    arrival: Time,
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
        s.insert_row(
            id,
            IdleRow {
                id: id.id(),
                location: c,
                arrival: Time(0.0),
            },
        );

        // panic!("{:#?}", s);
    }
}
