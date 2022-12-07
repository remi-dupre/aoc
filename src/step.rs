use std::fmt::Display;

/// A step that can't return an error
pub struct InfaillibleStep<F>(pub F);

/// A step that can convert buffer input
pub struct GeneratorStep<F>(pub F);

pub trait Step<I, O> {
    fn run(&self, input: I) -> Result<O, String>;
}

impl<F, FO, I, O> Step<I, O> for F
where
    F: Fn(I) -> FO + 'static,
    FO: StepResult<Val = O>,
{
    fn run(&self, input: I) -> Result<O, std::string::String> {
        self(input).into_result()
    }
}

impl<F, I, O> Step<I, O> for InfaillibleStep<F>
where
    F: Fn(I) -> O + 'static,
{
    fn run(&self, input: I) -> Result<O, std::string::String> {
        Ok((self.0)(input))
    }
}

/// Represent a feasible generator input
pub trait GeneratorInput<'a> {
    fn take_buffer(buffer: &'a mut String) -> Self;
}

impl<'a> GeneratorInput<'a> for &'a str {
    fn take_buffer(buffer: &'a mut String) -> Self {
        buffer
    }
}

impl<'a> GeneratorInput<'a> for &'a [u8] {
    fn take_buffer(buffer: &'a mut String) -> Self {
        buffer.as_bytes()
    }
}

impl GeneratorInput<'_> for String {
    fn take_buffer(buffer: &mut String) -> Self {
        std::mem::take(buffer)
    }
}

impl GeneratorInput<'_> for Vec<u8> {
    fn take_buffer(buffer: &mut String) -> Self {
        std::mem::take(buffer).into_bytes()
    }
}

/// Represent a faillible result that can be extracted in a generic way.
pub trait StepResult {
    type Val;
    fn into_result(self) -> Result<Self::Val, String>;
}

impl<T> StepResult for Option<T> {
    type Val = T;

    fn into_result(self) -> Result<Self::Val, String> {
        self.ok_or_else(|| "no output".to_string())
    }
}

impl<T, E: Display> StepResult for Result<T, E> {
    type Val = T;

    fn into_result(self) -> Result<Self::Val, String> {
        self.map_err(|err| err.to_string())
    }
}
