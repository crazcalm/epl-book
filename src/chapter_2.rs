trait Number {
    fn zero(&self) -> Self;
    fn is_zero(&self) -> bool;
    fn successor(&self) -> Self;
    fn predecessor(&self) -> Self;
}

type Int = i32;

impl Number for Int {
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

struct BigInt {
    #[allow(dead_code)]
    base: u32,

    #[allow(dead_code)]
    powers: Vec<u32>,
}

impl BigInt {
    #[allow(dead_code)]
    pub fn new(base: u32) -> Self {
        BigInt {
            base,
            powers: vec![],
        }
    }

    #[allow(dead_code)]
    pub fn value(&self) -> u32 {
        self.powers
            .iter()
            .enumerate()
            .map(|(index, num)| {
                if index == 0 {
                    num.clone()
                } else {
                    (1..index as usize).fold(0, |sum, _| sum + self.base)
                }
            })
            .fold(0, |sum, i| sum + i)
    }
}

impl Number for BigInt {
    fn zero(&self) -> Self {
        BigInt {
            base: 0,
            powers: vec![],
        }
    }

    fn is_zero(&self) -> bool {
        self.powers.is_empty()
    }

    fn successor(&self) -> Self {
        match self.powers.is_empty() {
            true => BigInt {
                base: self.base.clone(),
                powers: vec![],
            },
            false => {
                let mut new_powers = Vec::with_capacity(self.powers.len() + 1);

                for (index, num) in self.powers.iter().rev().enumerate() {
                    if num + 1 < self.base {
                        new_powers.insert(new_powers.len() - index - 1, num + 1)
                    } else {
                        new_powers.insert(new_powers.len() - index - 1, 0);
                        new_powers.insert(new_powers.len() - index - 2, 1)
                    }
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
                powers: vec![],
            },
            false => {
                let mut new_powers = Vec::with_capacity(self.powers.len());

                for (index, num) in self.powers.iter().enumerate() {
                    if num - 1 > 0 {
                        new_powers.insert(index, num - 1)
                    } else {
                        new_powers.insert(index, 0);
                        new_powers.insert(index + 1, self.base - 1)
                    }
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
    use super::*;

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
