#[cfg(feature = "i64")]
use collatz_iter::prelude::*;

#[cfg(feature = "i64")]
fn main() {
    // check middle signed number
    let number = NonZeroI64::new(
        12_555_797_i64
    ).unwrap();
    for n in number.collatz_iter() {
        println!("{:?}", n);
    }
    println!();
}

#[cfg(not(feature = "i64"))]
fn main() {
    println!("You need enable feature \"i64\"");
}
