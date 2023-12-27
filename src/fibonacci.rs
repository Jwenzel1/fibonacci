pub trait CheckedAdd
where
    Self: Sized,
{
    fn checked_add(self, other: Self) -> Option<Self>;
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

impl<T> Iterator for FibonacciSequence<T>
where
    T: CheckedAdd + Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let return_value: Option<T>;
        if self.current.is_some() {
            return_value = self.current;
            if self.next.is_some() {
                self.current = self.next;
                self.next = self.current.unwrap().checked_add(return_value.unwrap());
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
    use super::{FibonacciSequence, CheckedAdd};

    #[derive(Clone, Copy)]
    struct FibonacciNumber(u8);

    impl CheckedAdd for FibonacciNumber {
        fn checked_add(self, other: Self) -> Option<Self> {
            match self.0.checked_add(other.0) {
                Some(val) => Some(Self(val)),
                None => None
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
