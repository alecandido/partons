// TODO: to do even more and allow passing arguments, I believe I'd need a procedural macro
#[macro_export]
macro_rules! run {
    ($value:expr; $($sub:ident),*) => {
        {
            match $value {
                $(
                    Self::$sub(var) => var.run(),
                )*
            }
        }
    };
}
