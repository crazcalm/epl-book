use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::ops::Add;

pub trait Number {
    fn value(&self) -> i32;
    fn zero(&self) -> Self;
    fn is_zero(&self) -> bool;
    fn successor(&self) -> Self;
    fn predecessor(&self) -> Self;
}
#[allow(dead_code)]
pub fn factorial<
    T: Number
        + std::fmt::Debug
        + std::ops::Add<Output = T>
        + PartialOrd
        + Clone
        + std::ops::Mul<Output = T>,
>(
    num: T,
) -> <T as Add>::Output {
    let mut result = num.clone();
    let mut counter = num.clone().predecessor();

    while counter.is_zero() == false {
        result = counter.clone() * result;
        counter = counter.predecessor();
    }

    result
}

#[allow(dead_code)]
pub fn fib<T: Number + std::fmt::Debug + std::ops::Add<Output = T> + PartialOrd>(
    num: T,
) -> <T as Add>::Output {
    if num <= num.zero() {
        num.zero()
    } else if num == num.zero().successor() {
        num.zero().successor()
    } else if num == num.zero().successor().successor() {
        fib(num.predecessor()) + num.zero()
    } else {
        let result = fib(num.predecessor()) + fib(num.predecessor().predecessor());
        result
    }
}

type Int = i32;

impl Number for Int {
    fn value(&self) -> i32 {
        self.clone()
    }

    fn zero(&self) -> Self {
        0
    }

    fn is_zero(&self) -> bool {
        *self == 0
    }
    fn successor(&self) -> Self {
        *self + 1
    }

    fn predecessor(&self) -> Self {
        *self - 1
    }
}

#[derive(Debug, Clone)]
struct BigInt {
    #[allow(dead_code)]
    base: i32,

    #[allow(dead_code)]
    powers: Vec<i32>,
}

impl BigInt {
    #[allow(dead_code)]
    pub fn new(base: i32) -> Self {
        BigInt {
            base,
            powers: vec![],
        }
    }
}

impl Eq for BigInt {}

impl Ord for BigInt {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value().cmp(&other.value())
    }
}

impl PartialOrd for BigInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for BigInt {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}

impl Add for BigInt {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let mut result: BigInt;
        let mut counter: BigInt;

        match self.base == other.base {
            false => panic!("Incompatible bases"),
            true => {
                if self.value() > other.value() {
                    result = self.clone();
                    counter = other.clone();
                } else {
                    counter = self.clone();
                    result = other.clone();
                }
                while counter.is_zero() == false {
                    result = result.successor();
                    counter = counter.predecessor();
                }
            }
        }

        result
    }
}

impl Number for BigInt {
    fn value(&self) -> i32 {
        self.powers
            .iter()
            .rev()
            .enumerate()
            .map(|(index, num)| {
                if index == 0 {
                    num.clone()
                } else if index == 1 {
                    num.clone() * self.base
                } else {
                    let power = (1..=index as usize).fold(1, |sum, _| sum * self.base);
                    num.clone() * power
                }
            })
            .fold(0, |sum, i| sum + i)
    }

    fn zero(&self) -> Self {
        BigInt {
            base: 10,
            powers: vec![0],
        }
    }

    fn is_zero(&self) -> bool {
        self.powers.is_empty() || self.powers == vec![0]
    }

    fn successor(&self) -> Self {
        match self.powers.is_empty() {
            true => BigInt {
                base: self.base.clone(),
                powers: vec![0],
            },
            false => {
                let mut new_powers = self.powers.clone();

                for (index, num) in self.powers.iter().rev().enumerate() {
                    if num + 1 < self.base {
                        new_powers[self.powers.len() - index - 1] = num + 1;
                        break;
                    } else {
                        new_powers[self.powers.len() - index - 1] = 0;

                        if self.powers.len() > index + 1 {
                            new_powers[self.powers.len() - index - 2] = num + 1
                        } else {
                            new_powers.insert(0, 1);
                        }
                    }
                }

                // Get rid of extra powers
                if new_powers.len() > 1 && new_powers[0] == 0 {
                    let _ = new_powers.remove(0);
                }

                BigInt {
                    base: self.base.clone(),
                    powers: new_powers,
                }
            }
        }
    }

    fn predecessor(&self) -> Self {
        match self.powers.is_empty() {
            true => BigInt {
                base: self.base.clone(),
                powers: vec![0],
            },
            false => {
                let mut new_powers = self.powers.clone();

                for (index, num) in self.powers.iter().rev().enumerate() {
                    // Edge case:
                    if new_powers == vec![1] {
                        new_powers = vec![0]
                    }

                    if num - 1 >= 0 {
                        new_powers[self.powers.len() - index - 1] = num - 1;
                        break;
                    } else {
                        new_powers[self.powers.len() - index - 1] = self.base - 1;

                        if self.powers.len() - index - 2 > 0 {
                            let second_num = new_powers[self.powers.len() - index - 2];

                            if second_num > 0 {
                                new_powers[self.powers.len() - index - 2] = second_num - 1;
                                break;
                            } else {
                                continue;
                            }
                        }
                    }
                }

                // Get rid of extra powers
                if new_powers.len() > 1 && new_powers[0] == 0 {
                    let _ = new_powers.remove(0);
                }

                BigInt {
                    base: self.base.clone(),
                    powers: new_powers,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_add_for_int() {
        let one: Int = 1;
        let two: Int = 2;
        let three: Int = 3;

        assert_eq!(one + two, three);
    }

    #[test]
    fn test_add_for_big_int() {
        let ten = BigInt {
            base: 10,
            powers: vec![1, 0],
        };
        let hundred = BigInt {
            base: 10,
            powers: vec![1, 0, 0],
        };

        let result = ten.clone() + hundred.clone();
        assert_eq!(result.value(), 110);

        let result = hundred.clone() + ten.clone();
        assert_eq!(result.value(), 110);

        let result = hundred.clone() + hundred.clone();
        assert_eq!(result.value(), 200);
    }

    #[test]
    fn test_fib_for_int() {
        let ten: Int = 10;

        let result = fib(ten);

        assert_eq!(result, 55);
    }

    #[test]
    fn test_factorial_for_int() {
        let ten: Int = 10;

        let result = factorial(ten);

        assert_eq!(result, 3628800);
    }

    #[test]
    fn test_fib_for_big_int() {
        let ten = BigInt {
            base: 10,
            powers: vec![1, 0],
        };

        let result = fib(ten);

        assert_eq!(result.value(), 55);
    }

    #[test]
    fn test_bigint_is_zero() {
        let big_int = BigInt {
            base: 10,
            powers: vec![0],
        };

        assert_eq!(big_int.is_zero(), true);
    }

    #[test]
    fn test_bigint_predecessor() {
        let mut big_int = BigInt {
            base: 10,
            powers: vec![3, 0, 0],
        };

        for num in (0..300).rev() {
            big_int = big_int.predecessor();

            assert_eq!(big_int.value(), num);
        }
    }

    #[test]
    fn test_bigint_successor() {
        let mut big_int = BigInt {
            base: 10,
            powers: vec![0],
        };

        for num in 1..300 {
            big_int = big_int.successor();

            assert_eq!(big_int.value(), num);
        }
    }

    #[test]
    fn test_bigint_value() {
        let seven = BigInt {
            base: 10,
            powers: vec![7],
        };

        assert_eq!(seven.value(), 7);

        let twenty_seven = BigInt {
            base: 10,
            powers: vec![2, 7],
        };

        assert_eq!(twenty_seven.value(), 27);

        let three_hundred_seven = BigInt {
            base: 10,
            powers: vec![3, 0, 7],
        };

        assert_eq!(three_hundred_seven.value(), 307);
    }

    #[test]
    fn test_int_zero() {
        let num: Int = 5;

        assert_eq!(num.zero(), 0);
    }

    #[test]
    fn test_int_is_zero() {
        let num: Int = 0;
        assert_eq!(num.is_zero(), true);

        let num: Int = 1;
        assert_eq!(num.is_zero(), false);
    }

    #[test]
    fn test_int_successor() {
        let num: Int = 4;

        assert_eq!(num.successor(), 5);
    }

    #[test]
    fn test_int_predecessor() {
        let num: Int = 4;

        assert_eq!(num.predecessor(), 3);
    }
}
