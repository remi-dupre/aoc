use std::fmt::Display;

pub trait TryUnwrap {
    type Val;
    fn try_unwrap(self) -> Result<Self::Val, String>;
}

impl<T> TryUnwrap for Option<T> {
    type Val = T;

    fn try_unwrap(self) -> Result<Self::Val, String> {
        if let Some(val) = self {
            Ok(val)
        } else {
            Err("empty output".to_string())
        }
    }
}

impl<T, E: Display> TryUnwrap for Result<T, E> {
    type Val = T;

    fn try_unwrap(self) -> Result<Self::Val, String> {
        self.map_err(|err| format!("{}", err))
    }
}
