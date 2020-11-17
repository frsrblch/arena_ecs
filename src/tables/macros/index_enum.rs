#[macro_export]
macro_rules! index_enum {
    (
        enum $name:ident {
            $( $variant:ident, )*
        }
    ) => {
        #[derive(Debug, Eq, PartialEq, Hash)]
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
