use std::num::NonZeroU128;

pub trait CollatzIteratorU128 {
    type Iterator;
    fn collatz_iter(&self) -> Self::Iterator;
}

impl CollatzIteratorU128 for NonZeroU128 {
    type Iterator = CollatzIterU128;
    fn collatz_iter(&self) -> Self::Iterator {
        CollatzIterU128(self.clone().get())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CollatzIterU128(u128);

impl Iterator for CollatzIterU128 {
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
