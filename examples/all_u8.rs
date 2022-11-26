use collatz_iter::prelude::*;

fn main() {
    // check all numbers in u8
    for number in 1..=u8::MAX {
        for n in NonZeroU8::new(number).unwrap().collatz_iter() {
            print!("{:?} ", n);
        }
        println!();
    }
}
