use std::io::{self, Error, ErrorKind::Other};

use collatz_conjecture::prelude::*;

fn main() -> io::Result<()> {
    let mut collatz_iter = NonZeroU128::new(
        1024_u128.pow(12_u32) * 255 // + 1 // with +1 will overflow on first step
    ).unwrap().collatz_iter();
    println!("Test: {:?}", collatz_iter);

    loop {
        let n = collatz_iter.next()
            .ok_or(Error::new(Other, "Overflow u128!"))?;

        println!("{}", n);
        if n == NonZeroU128::new(1).unwrap() {
            println!("That's cycle 4 -> 2 -> 1!");
            break;
        }
    }

    let mut collatz_iter = NonZeroU16::new(2_u16.pow(5) - 1).unwrap().collatz_iter();
    println!("Test: {:?}", collatz_iter);

    loop {
        let n = collatz_iter.next()
            .ok_or(Error::new(Other, "Overflow u16!"))?;

        println!("{}", n);
        if n == NonZeroU16::new(1).unwrap() {
            println!("That's cycle 4 -> 2 -> 1!");
            break;
        }
    }

    Ok(())
}
