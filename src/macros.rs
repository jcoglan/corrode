macro_rules! wrapper_enum {
    ($name:ident {
        $( $field:ident -> $type:ident ),+
        $(,)*
    }) => {
        pub enum $name {
            $( $type($type), )+
        }

        impl $name {
            $(
                #[allow(dead_code)]
                pub fn $field(self) -> Option<$type> {
                    match self {
                        $name::$type($field) => Some($field),
                        _ => None,
                    }
                }
            )+
        }

        $(
            impl From<$type> for $name {
                fn from($field: $type) -> Self {
                    $name::$type($field)
                }
            }
        )+
    }
}
