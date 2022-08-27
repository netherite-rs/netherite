#[cfg(feature = "enum_utils_derive")]
#[allow(unused_imports)]
#[macro_use]
extern crate enum_utils_derive;

#[cfg(feature = "enum_utils_derive")]
#[doc(hidden)]
pub use enum_utils_derive::*;

/// A trait used by enums to give them a compile-time name.
pub trait NamedEnum {
    fn name(&self) -> &'static str;
    fn from_name(name: &str) -> Result<Self, String> where Self: Sized;
}

/// A trait used by enums to give them an ordinal number
pub trait OrdinalEnum {
    fn ordinal(&self) -> u32;
    fn from_ordinal(ordinal: u32) -> Result<Self, String> where Self: Sized;
}