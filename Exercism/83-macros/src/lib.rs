#[macro_export]
macro_rules! hashmap {
    (
        $($key:expr => $value: expr),*
        $(,)?
    ) => {{
        let mut hash = ::std::collections::HashMap::new();
        $(
            hash.insert($key, $value);
        )*
        hash
    }};
}
