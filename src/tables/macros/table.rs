#[macro_export]
macro_rules! table {
    (
        struct $table:ident {
            type Arena = $arena:ident;
            type Row = struct $row:ident;
            type Index = $state_index:ident;
            fields {
                $( $field:ident: $t:ty, )* $(,)?
            }
            links {
                $( $link:ident: $a:ty, )* $(,)?
            }
        }
    ) => {
        #[derive(Debug)]
        pub struct $table {
            pub id: $crate::Column<Self, $crate::Id<$arena>>,
            $(
                pub $field: $crate::Column<Self, $t>,
            )*
            $(
                pub $link: $crate::IdColumn<Self, $a>,
            )*
        }

        impl Default for $table {
            fn default() -> Self {
                Self {
                    id: Default::default(),
                    $(
                        $field: Default::default(),
                    )*
                    $(
                        $link: Default::default(),
                    )*
                }
            }
        }

        #[allow(dead_code)]
        impl $table {
            fn insert<'a>(
                &mut self,
                row: Valid<'a, $row>,
                indices: &mut $crate::IdIndices<$arena, $state_index>
            ) {
                let id = $row::id(&row);
                let index = self.insert_inner(row.value);
                indices.insert(id, index);
            }

            fn insert_inner(&mut self, row: $row) -> $crate::Index<Self> {
                $(
                    self.$field.push(row.$field);
                )*
                $(
                    self.$link.push(row.$link.map(Valid::assert));
                )*
                self.id.push(row.id)
            }

            fn swap_remove(
                &mut self,
                index: $crate::Index<Self>,
                indices: &mut $crate::IdIndices<$arena, $state_index>
            ) -> $row {
                let row = self.swap_remove_inner(&index);
                if let Some(swapped) = self.id.get(&index).map($crate::Valid::assert) {
                    indices.insert(swapped, index);
                }
                row
            }

            fn swap_remove_inner(&mut self, index: &$crate::Index<Self>) -> $row {
                $row {
                    id: self.id.swap_remove(index),
                    $(
                        $field: self.$field.swap_remove(index),
                    )*
                    $(
                        $link: self.$link.swap_remove(index),
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
            id: $crate::Id<$arena>,
            $(
                $field: $t,
            )*
            $(
                $link: Option<$crate::Id<$a>>,
            )*
        }

        impl $row {
            pub fn new<'a>(
                id: impl $crate::ValidId<$arena> + 'a,
                $(
                    $field: $t,
                )*
                $(
                    $link: impl $crate::ValidId<$a> + 'a,
                )*
            ) -> Valid<'a, Self> {
                let value = Self {
                    id: id.id(),
                    $(
                        $field,
                    )*
                    $(
                        $link: Some($link.id()),
                    )*
                };
                Valid::assert(value)
            }

            pub fn id<'a>(row: &Valid<'a, $row>) -> Valid<'a, Id<$arena>> {
                Valid::assert(row.value.id)
            }

            $(
                pub fn $link<'a>(row: &Valid<'a, $row>) -> Option<Valid<'a, Id<$a>>> {
                    row.value.$link.map(Valid::assert)
                }
            )*
        }
    }
}
