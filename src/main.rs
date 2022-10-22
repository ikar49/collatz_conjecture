use std::io;
use std::num::NonZeroU128;

#[derive(Debug, Clone, Copy)]
struct CollatzIterator(u128);

impl CollatzIterator {
    fn new(n: NonZeroU128) -> Self {
        Self(n.get())
    }
}

impl Iterator for CollatzIterator {
    type Item = NonZeroU128;

    fn next(&mut self) -> Option<Self::Item> {
        let result = NonZeroU128::new(self.0.clone());

        if &self.0 % 2 == 0 {
            self.0 = self.0 / 2;
        } else {
            self.0 = self.0.checked_mul(3)
                            // Записываем MAX, чтобы
                           .unwrap_or(u128::MAX)
                            // вызвать второе переполнение при сложении, чтобы
                           .checked_add(1)
                            // точно записать 0 в self
                           .unwrap_or(0);
        }

        result
    }
}

fn main() -> io::Result<()> {
    let mut collatz_iter = CollatzIterator::new(NonZeroU128::new(
        1024_u128.pow(12_u32) * 255 // + 1 // with +1 will overflow on first step
    ).unwrap());
    println!("Test: {:?}", collatz_iter);
    loop {
        let n = collatz_iter.next()
            .ok_or(io::Error::new(io::ErrorKind::Other, "Overflow u128!"))?;

        println!("{}", n);
        if n == NonZeroU128::new(1).unwrap() {
            println!("That's cycle 4 -> 2 -> 1!");
            break;
        }
    }

    Ok(())
}
