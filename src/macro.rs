macro_rules! collatz {
    ($iter_name:ident($int_name:ty) for $nonzero_name:ident) => (
        use std::num::$nonzero_name;
        use crate::CollatzIterator;

        impl CollatzIterator for $nonzero_name {
            type Iterator = $iter_name;
            fn collatz_iter(&self) -> Self::Iterator {
                $iter_name(Some(self.clone().get()))
            }
        }

        #[derive(Debug, Clone, Copy)]
        pub struct $iter_name(Option<$int_name>);

        impl Iterator for $iter_name {
            type Item = Result<$nonzero_name, &'static str>;

            fn next(&mut self) -> Option<Self::Item> {
                if let None = self.0 {
                    // На предыдущем шаге мы достигли 1, а значит попали в цикл 4 -> 2 -> 1 -> 4...
                    // Или на предыдущем шаге мы уже вернули ошибку переполнения целого
                    return None;
                }

                // safety: Мы уже проверяли на None выше
                let val = unsafe { self.0.clone().unwrap_unchecked() };
                if val == 0 {
                    // На предыдущем шаге произошло переполнение
                    // Следующая итерация не должна случиться
                    self.0 = None;
                    return Some(Err("Integer Overflow"));
                }

                // Надеюсь что компилятор оптимизирует для unsigned типов эту проверку
                #[allow(unused_comparisons)]
                if val < 0 {
                    // Итератор инициализировали отрицательным числом
                    // Следующая итерация не должна случиться
                    self.0 = None;
                    return Some(Err("Negative Integer"));
                }

                // safety: Мы уже проверяли на ноль выше
                let result = Ok(unsafe { <$nonzero_name>::new_unchecked(val) });

                self.0 = if val != 1 {
                    Some(if &val % 2 == 0 {
                        val / 2
                    } else {
                        val.checked_mul(3)              // При переполнении
                           .unwrap_or(<$int_name>::MAX) // записываем MAX, чтобы
                           .checked_add(1)              // вызвать второе переполнение при сложении,
                           .unwrap_or(0)                // чтобы точно записать 0
                    })
                } else {
                    // Так как текущее значение 1 - его всё равно надо вывести,
                    // но следующей итерации произойти не должно
                    None
                };

                Some(result)
            }
        }
    );
}
