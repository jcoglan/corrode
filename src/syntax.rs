macro_rules! delegate_trait {
    ($trait:ident, $( $e:tt )+) => {
        trait_defn!(define_trait_with_deps, ($( $e )+), $trait);
    };
}

macro_rules! define_trait_with_deps {
    (
        $e:tt,
        $trait:ident,
        [$( $dep_trait:ident ),*],
        $methods:tt
    ) => {
        $(
            trait_defn!(define_trait, $e, $dep_trait);
        )*
        impl_trait!($e, $trait, $methods);
    };
}

macro_rules! define_trait {
    ($e:tt, $trait:ident, $deps:tt, $methods:tt) => {
        impl_trait!($e, $trait, $methods);
    };
}

macro_rules! impl_trait {
    (
        ($type:ident => $field:ident),
        $trait:ident,
        $methods:tt
    ) => {
        impl_methods!((impl $trait for $type), $methods, $field);
    };

    (
        ($type:ident<$( $gen:ident ),+> => $field:ident),
        $trait:ident,
        $methods:tt
    ) => {
        impl_methods!(
            (impl<$( $gen: $trait ),+> $trait for $type<$( $gen ),+>),
            $methods,
            $field
        );
    };
}

macro_rules! impl_methods {
    (
        ($( $impl:tt )+),
        { $( $method:ident -> $ret:ty ),* },
        $field:ident
    ) => {
        $( $impl )+ {
            $(
                fn $method(&self, other: &Self) -> $ret {
                    self.$field.$method(&other.$field)
                }
            )*
        }
    };
}

macro_rules! trait_defn {
    ($m:ident, $e:tt, PartialOrd) => {
        $m!($e, PartialOrd,
            [PartialEq],
            { partial_cmp -> Option<std::cmp::Ordering> });
    };
    ($m:ident, $e:tt, Ord) => {
        $m!($e, Ord,
            [PartialOrd, Eq, PartialEq],
            { cmp -> std::cmp::Ordering });
    };
    ($m:ident, $e:tt, PartialEq) => {
        $m!($e, PartialEq,
            [],
            { eq -> bool });
    };
    ($m:ident, $e:tt, Eq) => {
        $m!($e, Eq,
            [PartialEq],
            {});
    };
}
