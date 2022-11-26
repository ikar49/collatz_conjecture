use collatz_iter::prelude::*;

fn main() {
    // check big unsigned number
    let number = NonZeroU128::new(
        1024_u128.pow(12_u32) * 255 // with +1 will overflow on first step
    ).unwrap();
    for n in number.collatz_iter() {
        println!("{:?}", n);
    }
    println!();
}
