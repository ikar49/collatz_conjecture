pub trait CollatzIterator {
    type Iterator;
    fn collatz_iter(&self) -> Self::Iterator;
}

#[macro_use]
mod r#macro;

#[cfg(feature = "u8")]
pub mod r#u8 {
    collatz!(CollatzIterU8(u8) for NonZeroU8);
}

#[cfg(feature = "u16")]
pub mod r#u16 {
    collatz!(CollatzIterU16(u16) for NonZeroU16);
}

#[cfg(feature = "u32")]
pub mod r#u32 {
    collatz!(CollatzIterU32(u32) for NonZeroU32);
}

#[cfg(feature = "u64")]
pub mod r#u64 {
    collatz!(CollatzIterU64(u64) for NonZeroU64);
}

#[cfg(feature = "u128")]
pub mod r#u128 {
    collatz!(CollatzIterU128(u128) for NonZeroU128);
}

#[cfg(feature = "usize")]
pub mod r#usize {
    collatz!(CollatzIterUsize(usize) for NonZeroUsize);
}

#[cfg(feature = "i8")]
pub mod r#i8 {
    collatz!(CollatzIterI8(i8) for NonZeroI8);
}

#[cfg(feature = "i16")]
pub mod r#i16 {
    collatz!(CollatzIterI16(i16) for NonZeroI16);
}

#[cfg(feature = "i32")]
pub mod r#i32 {
    collatz!(CollatzIterI32(i32) for NonZeroI32);
}

#[cfg(feature = "i64")]
pub mod r#i64 {
    collatz!(CollatzIterI64(i64) for NonZeroI64);
}

#[cfg(feature = "i128")]
pub mod r#i128 {
    collatz!(CollatzIterI128(i128) for NonZeroI128);
}

#[cfg(feature = "isize")]
pub mod r#isize {
    collatz!(CollatzIterIsize(isize) for NonZeroIsize);
}

pub mod prelude;
