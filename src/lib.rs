pub mod prelude;

#[macro_use]
mod r#macro;

pub mod collatz_u8 {
    collatz!(CollatzIteratorU8, CollatzIterU8, NonZeroU8, u8);
}

pub mod collatz_u16 {
    collatz!(CollatzIteratorU16, CollatzIterU16, NonZeroU16, u16);
}

pub mod collatz_u32 {
    collatz!(CollatzIteratorU32, CollatzIterU32, NonZeroU32, u32);
}

pub mod collatz_u64 {
    collatz!(CollatzIteratorU64, CollatzIterU64, NonZeroU64, u64);
}

pub mod collatz_u128 {
    collatz!(CollatzIteratorU128, CollatzIterU128, NonZeroU128, u128);
}

pub mod collatz_usize {
    collatz!(CollatzIteratorUsize, CollatzIterUsize, NonZeroUsize, usize);
}

pub mod collatz_i8 {
    collatz!(CollatzIteratorI8, CollatzIterI8, NonZeroI8, i8);
}

pub mod collatz_i16 {
    collatz!(CollatzIteratorI16, CollatzIterI16, NonZeroI16, i16);
}

pub mod collatz_i32 {
    collatz!(CollatzIteratorI32, CollatzIterI32, NonZeroI32, i32);
}

pub mod collatz_i64 {
    collatz!(CollatzIteratorI64, CollatzIterI64, NonZeroI64, i64);
}

pub mod collatz_i128 {
    collatz!(CollatzIteratorI128, CollatzIterI128, NonZeroI128, i128);
}

pub mod collatz_isize {
    collatz!(CollatzIteratorIsize, CollatzIterIsize, NonZeroIsize, isize);
}
