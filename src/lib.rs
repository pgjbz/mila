pub mod ast;
pub mod evaluator;
pub mod lexer;

pub mod parser;

#[macro_export]
macro_rules! hashmap {
    ($tkey:ty : $tval:ty, $($key:expr => $value:expr),*) => {
        {
            let mut hashmap: HashMap<$tkey, $tval> = std::collections::HashMap::new();
            $(
                hashmap.insert($key.to_string(), $value);
            )*
            hashmap
        }
    };
}