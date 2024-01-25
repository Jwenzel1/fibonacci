pub trait CheckedAdd<T = Self>
where
    Self: Sized,
{
    fn checked_add(self, other: T) -> Option<Self>;
}

pub struct FibonacciSequence<T> {
    current: Option<T>,
    next: Option<T>,
}

impl<T> FibonacciSequence<T> {
    pub fn new(start: T, next: T) -> Self {
        Self {
            current: Some(start),
            next: Some(next),
        }
    }
}

impl<T: CheckedAdd + Copy> Iterator for FibonacciSequence<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let return_value;
        if let Some(current_val) = self.current {
            return_value = self.current;
            if let Some(next_val) = self.next {
                self.current = self.next;
                self.next = current_val.checked_add(next_val);
            } else {
                self.current = None;
            }
        } else {
            return_value = None;
        }
        return_value
    }
}

#[cfg(test)]
mod tests {
    use super::{CheckedAdd, FibonacciSequence};

    #[derive(Clone, Copy)]
    struct FibonacciNumber(u8);

    impl CheckedAdd for FibonacciNumber {
        fn checked_add(self, other: Self) -> Option<Self> {
            match self.0.checked_add(other.0) {
                Some(val) => Some(Self(val)),
                None => None,
            }
        }
    }

    enum KnownFibonacciNumber {
        First,
        Second,
        Third,
        Fifth,
    }

    impl KnownFibonacciNumber {
        fn value(self) -> FibonacciNumber {
            match self {
                Self::First => FibonacciNumber(0),
                Self::Second => FibonacciNumber(1),
                Self::Third => FibonacciNumber(1),
                Self::Fifth => FibonacciNumber(3),
            }
        }
    }

    fn make_test_sequence() -> FibonacciSequence<FibonacciNumber> {
        FibonacciSequence::new(
            KnownFibonacciNumber::First.value(),
            KnownFibonacciNumber::Second.value(),
        )
    }

    #[test]
    fn calculates_first_num_correctly() {
        assert_eq!(
            KnownFibonacciNumber::First.value().0,
            make_test_sequence().into_iter().nth(0).unwrap().0
        );
    }

    #[test]
    fn calculates_second_num_correctly() {
        assert_eq!(
            KnownFibonacciNumber::Second.value().0,
            make_test_sequence().into_iter().nth(1).unwrap().0
        );
    }

    #[test]
    fn calculates_third_num_correctly() {
        assert_eq!(
            KnownFibonacciNumber::Third.value().0,
            make_test_sequence().into_iter().nth(2).unwrap().0
        );
    }

    #[test]
    fn calculates_fifth_num_correctly() {
        assert_eq!(
            KnownFibonacciNumber::Fifth.value().0,
            make_test_sequence().into_iter().nth(4).unwrap().0
        );
    }
}
