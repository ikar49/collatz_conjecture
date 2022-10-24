//macro_rules! collatz_internal {
//    (ns $)
//}

macro_rules! collatz {
    ($trait_collatz_name:ident, $struct_collatz_name:ident, $nonzero_name:ident, $int_name:ty) => (
        use std::num::$nonzero_name;

        pub trait $trait_collatz_name {
            type Iterator;
            fn collatz_iter(&self) -> Self::Iterator;
        }

        impl $trait_collatz_name for $nonzero_name {
            type Iterator = $struct_collatz_name;
            fn collatz_iter(&self) -> Self::Iterator {
                $struct_collatz_name(self.clone().get())
            }
        }

        #[derive(Debug, Clone, Copy)]
        pub struct $struct_collatz_name($int_name);

        impl Iterator for $struct_collatz_name {
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
