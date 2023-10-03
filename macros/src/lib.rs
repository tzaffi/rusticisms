#[macro_export]
macro_rules! hashmap {
    () => {
        {
            ::std::collections::HashMap::new()
        }
    };
    ($($key:expr => $val:expr),+ $(,)?) => {
        {
            let mut hm = ::std::collections::HashMap::new();
            $(
                hm.insert($key, $val);
            )+
            hm
        }
    };
}
