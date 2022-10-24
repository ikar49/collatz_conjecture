pub use crate::CollatzIterator;

#[cfg(feature = "u8")]
pub use std::num::NonZeroU8;

#[cfg(feature = "u16")]
pub use std::num::NonZeroU16;

#[cfg(feature = "u32")]
pub use std::num::NonZeroU32;

#[cfg(feature = "u64")]
pub use std::num::NonZeroU64;

#[cfg(feature = "u128")]
pub use std::num::NonZeroU128;

#[cfg(feature = "usize")]
pub use std::num::NonZeroUsize;

#[cfg(feature = "i8")]
pub use std::num::NonZeroI8;

#[cfg(feature = "i16")]
pub use std::num::NonZeroI16;

#[cfg(feature = "i32")]
pub use std::num::NonZeroI32;

#[cfg(feature = "i64")]
pub use std::num::NonZeroI64;

#[cfg(feature = "i128")]
pub use std::num::NonZeroI128;

#[cfg(feature = "isize")]
pub use std::num::NonZeroIsize;
