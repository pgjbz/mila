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

#[macro_export]
macro_rules! downcast {
    ($val:expr => $ty:ty) => {
        $val.as_any().downcast_ref::<$ty>()
    };
}

#[macro_export]
macro_rules! downcast_any {
    ($val:expr => $ty:ty) => {
        crate::downcast!($val => $ty).unwrap()
    };
}

#[macro_export]
macro_rules! downcast_option  {
    ($val:expr => $ty:ty) => {
        crate::downcast!($val.unwrap() => $ty)
    };
}
