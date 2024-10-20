#[macro_export]
macro_rules! hashmap {
    () => {{
        ::std::collections::HashMap::new()
    }};
    (
        $($key:expr => $value: expr),+ // atleast 1 expr must be present
        $(,)? // last element may or may not have `,`
    ) => {{
        let mut hash = ::std::collections::HashMap::new();
        $(
            hash.insert($key, $value);
        )*
        hash
    }};
}
