macro_rules! magic {
    ($($name:ident = $value:literal;)*) => {
        $(
            pub(crate) const $name: &str = $value;
        )*
    }
}
