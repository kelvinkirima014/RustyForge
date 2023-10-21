use std::time::SystemTime;

///The Core Rustyforge API
/// 
/// Resource allows us to handle different kinds of resources(like HTML,
/// CSS, Images, Json, etc.) in a more uniform way.
trait Resource {
    ///allows each implementor of `Resource` to specify
    /// what kind of output they produce.
    type Output;
    ///Return whether the resource has been modified or not
    fn modified(&self) -> Modified;
    ///actually produces the output
    fn generate(&self) -> Self::Output;
}

///Indicates the modification state of a `Resource`
pub enum Modified {
    Never,
    At(SystemTime)
}