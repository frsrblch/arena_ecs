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
            impl<'a> From<$v_row> for $name {
                fn from(value: $v_row) -> Self {
                    Self::$variant(value)
                }
            }
        )*
    }
}
