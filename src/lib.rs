pub mod ast;
pub mod evaluator;
pub mod lexer;

pub mod parser;

#[macro_export]
macro_rules! builtin_map {
    ($($key:expr => $value:expr),*) => {
        {
            let mut hashmap: HashMap<_, ObjectRef> = std::collections::HashMap::new();
            $(
                hashmap.insert($key.to_string(), $value);
            )*
            hashmap
        }
    };
}
