// Example code found at https://buterajay.medium.com/a-rust-koan-c06891dc35af

#[allow(dead_code)]
struct List<T> {
    pub cdr: Option<Box<List<T>>>,
    pub car: T,
}

impl<T> List<T> {
    #[allow(dead_code)]
    pub fn append(mut self, value: T) -> Self {
        match self.cdr {
            Some(list) => list.append(value),
            None => {
                self.cdr = Some(Box::new(List {
                    car: value,
                    cdr: None,
                }));
                self
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append() {
        let mut list: List<i32> = List {
            car: 7,
            cdr: Some(Box::new(List { car: 13, cdr: None })),
        };

        list = list.append(16);

        assert_eq!(list.cdr.unwrap().car, 16);
    }
}
