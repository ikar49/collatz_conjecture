use std::io;

#[derive(Debug, Clone, Copy)]
struct CollatzIterator(u128);

impl CollatzIterator {
    fn new(n: u128) -> Self {
        Self(n)
    }
}

impl Iterator for CollatzIterator {
    type Item = u128;

    fn next(&mut self) -> Option<Self::Item> {
        let result = if self.0 == 0 {
            None
        } else {
            Some(self.0.clone())
        };

        if self.0 % 2 == 0 {
            self.0 = self.0 / 2;
        } else {
            self.0 = self.0.checked_mul(3)
                           .unwrap_or(u128::MAX)
                            // Хотим вызвать второе переполнение при сложении,
                           .checked_add(1)
                            // чтобы точно записать 0
                           .unwrap_or(0);
        }

        result
    }
}

fn main() -> io::Result<()> {
    let mut collatz_iter = CollatzIterator::new(
        1024_u128.pow(12_u32) * 255 /*+ 1*/ // with +1 will overflow on first step
    );
    println!("Test: {:?}", collatz_iter);
    loop {
        let n = collatz_iter.next()
            .ok_or(io::Error::new(io::ErrorKind::Other, "Overflow u128!"))?;

        println!("{}", n);
        if n == 1 {
            println!("That's cycle 4 -> 2 -> 1!");
            break;
        }
    }

    Ok(())
}
