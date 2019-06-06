macro_rules! delegate_trait {
    ($trait:ident, $( $e:tt )+) => {
        delegate_trait!(@defn[define_trait_with_deps] ($( $e )+), $trait);
    };

    (
        @define_trait_with_deps
        $e:tt,
        $trait:ident,
        [$( $dep_trait:ident ),*],
        $methods:tt
    ) => {
        $(
            delegate_trait!(@defn[define_trait] $e, $dep_trait);
        )*
        delegate_trait!(@impl_trait $e, $trait, $methods);
    };

    (
        @define_trait
        $e:tt,
        $trait:ident,
        $deps:tt,
        $methods:tt
    ) => {
        delegate_trait!(@impl_trait $e, $trait, $methods);
    };

    (
        @impl_trait
        ($type:ident => $field:ident),
        $trait:ident,
        $methods:tt
    ) => {
        impl $trait for $type {
            delegate_trait!(@impl_methods $methods, $field);
        }
    };

    (
        @impl_trait
        ($type:ident<$( $gen:ident ),+> => $field:ident),
        $trait:ident,
        $methods:tt
    ) => {
        impl<$( $gen: $trait ),+> $trait for $type<$( $gen ),+> {
            delegate_trait!(@impl_methods $methods, $field);
        }
    };

    (
        @impl_methods
        { $( $method:ident -> $ret:ty ),* },
        $field:ident
    ) => {
        $(
            fn $method(&self, other: &Self) -> $ret {
                self.$field.$method(&other.$field)
            }
        )*
    };

    (@defn[$m:ident] $e:tt, PartialOrd) => {
        delegate_trait!(@$m $e, PartialOrd,
            [PartialEq],
            { partial_cmp -> Option<std::cmp::Ordering> });
    };
    (@defn[$m:ident] $e:tt, Ord) => {
        delegate_trait!(@$m $e, Ord,
            [PartialOrd, Eq, PartialEq],
            { cmp -> std::cmp::Ordering });
    };
    (@defn[$m:ident] $e:tt, PartialEq) => {
        delegate_trait!(@$m $e, PartialEq,
            [],
            { eq -> bool });
    };
    (@defn[$m:ident] $e:tt, Eq) => {
        delegate_trait!(@$m $e, Eq,
            [PartialEq],
            {});
    };
}
