use std::num::NonZeroU8;

pub trait CollatzIteratorU8 {
    type Iterator;
    fn collatz_iter(&self) -> Self::Iterator;
}

impl CollatzIteratorU8 for NonZeroU8 {
    type Iterator = CollatzIterU8;
    fn collatz_iter(&self) -> Self::Iterator {
        CollatzIterU8(self.clone().get())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CollatzIterU8(u8);

impl Iterator for CollatzIterU8 {
    type Item = NonZeroU8;

    fn next(&mut self) -> Option<Self::Item> {
        let result = NonZeroU8::new(self.0.clone());

        if &self.0 % 2 == 0 {
            self.0 = self.0 / 2;
        } else {
            self.0 = self.0.checked_mul(3)
                            // Записываем MAX, чтобы
                           .unwrap_or(u8::MAX)
                            // вызвать второе переполнение при сложении, чтобы
                           .checked_add(1)
                            // точно записать 0 в self
                           .unwrap_or(0);
        }

        result
    }
}
