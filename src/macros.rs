// Implements both the Display and the Serialize trait
// to use the provided closure function
macro_rules! serde_string_serializer_impl {
    ($name:ident, $var:ident, $expr:expr) => {
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                let $var = self;
                f.write_str(&$expr)
            }
        }

        impl ::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                serializer.collect_str(self)
            }
        }
    };
}