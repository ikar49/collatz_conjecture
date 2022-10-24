macro_rules! collatz {
    ($iter_name:ident($int_name:ty) for $nonzero_name:ident) => (
        use std::num::$nonzero_name;
        use crate::CollatzIterator;

        impl CollatzIterator for $nonzero_name {
            type Iterator = $iter_name;
            fn collatz_iter(&self) -> Self::Iterator {
                $iter_name(self.clone().get())
            }
        }

        #[derive(Debug, Clone, Copy)]
        pub struct $iter_name($int_name);

        impl Iterator for $iter_name {
            type Item = $nonzero_name;

            fn next(&mut self) -> Option<Self::Item> {
                let result = <$nonzero_name>::new(self.0.clone());

                if &self.0 % 2 == 0 {
                    self.0 = self.0 / 2;
                } else {
                    self.0 = self.0.checked_mul(3)
                                    // Записываем MAX, чтобы
                                   .unwrap_or(<$int_name>::MAX)
                                    // вызвать второе переполнение при сложении, чтобы
                                   .checked_add(1)
                                    // точно записать 0 в self
                                   .unwrap_or(0);
                }

                result
            }
        }
    );
}
