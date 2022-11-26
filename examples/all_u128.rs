use collatz_iter::prelude::*;

fn main() {
    // check all numbers in u128
    // NOT RECOMMENDED FOR RUN! It will be a very long time
    for number in 1..=u128::MAX {
        let number = NonZeroU128::new(number).unwrap();
        for n in number.collatz_iter() {
            if let Ok(n) = n {
                print!("{} ", n);
            } else {
                print!("!");
            }
        }
        println!();
    }
}
