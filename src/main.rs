use collatz_iter::prelude::*;

fn main() {
    // check big unsigned number
    let number = NonZeroU128::new(
        1024_u128.pow(12_u32) * 255 // + 1 // with +1 will overflow on first step
    ).unwrap();
    for n in number.collatz_iter() {
        println!("{:?}", n);
    }
    println!();

    // check middle signed number (need i64 feature)
    //let number = NonZeroI64::new(
    //    12_555_797_i64
    //).unwrap();
    //for n in number.collatz_iter() {
    //    println!("{:?}", n);
    //}
    //println!();

    // check all numbers in u8
    for number in 1..=u8::MAX {
        for n in NonZeroU8::new(number).unwrap().collatz_iter() {
            print!("{:?} ", n);
        }
        println!();
    }

    // check all numbers in u128 (not recommended)
    //for number in 1..=u128::MAX {
    //    let number = NonZeroU128::new(number).unwrap();
    //    for n in number.collatz_iter() {
    //        if let Ok(n) = n {
    //            print!("{} ", n);
    //        } else {
    //            print!("!");
    //        }
    //    }
    //    println!();
    //}
}
