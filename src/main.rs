mod fibonacci;

use fibonacci::{CheckedAdd, FibonacciSequence};

type FibonacciNumber = u128;

impl CheckedAdd for FibonacciNumber {
    fn checked_add(self, other: Self) -> Option<Self> {
        FibonacciNumber::checked_add(self, other)
    }
}

fn main() {
    let f: FibonacciSequence<FibonacciNumber> = FibonacciSequence::new(0, 1);
    for (i, value) in f.into_iter().enumerate() {
        println!("{}. {value}", i + 1)
    }
}
