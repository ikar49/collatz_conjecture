pub mod prelude;

pub trait CollatzIterator {
    type Iterator;
    fn collatz_iter(&self) -> Self::Iterator;
}

#[macro_use]
mod r#macro;

pub mod r#u8 {
    collatz!(CollatzIterU8(u8) for NonZeroU8);
}

pub mod r#u16 {
    collatz!(CollatzIterU16(u16) for NonZeroU16);
}

pub mod r#u32 {
    collatz!(CollatzIterU32(u32) for NonZeroU32);
}

pub mod r#u64 {
    collatz!(CollatzIterU64(u64) for NonZeroU64);
}

pub mod r#u128 {
    collatz!(CollatzIterU128(u128) for NonZeroU128);
}

pub mod r#usize {
    collatz!(CollatzIterUsize(usize) for NonZeroUsize);
}

pub mod r#i8 {
    collatz!(CollatzIterI8(i8) for NonZeroI8);
}

pub mod r#i16 {
    collatz!(CollatzIterI16(i16) for NonZeroI16);
}

pub mod r#i32 {
    collatz!(CollatzIterI32(i32) for NonZeroI32);
}

pub mod r#i64 {
    collatz!(CollatzIterI64(i64) for NonZeroI64);
}

pub mod r#i128 {
    collatz!(CollatzIterI128(i128) for NonZeroI128);
}

pub mod r#isize {
    collatz!(CollatzIterIsize(isize) for NonZeroIsize);
}
