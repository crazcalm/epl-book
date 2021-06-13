#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum List<T> {
    Nil,
    Cons(T, Box<List<T>>),
}

impl<T: Clone> List<T> {
    #[allow(dead_code)]
    pub fn car(&self) -> Option<T> {
        match self {
            List::Nil => None,
            List::Cons(car, _) => Some(car.clone()),
        }
    }
}

#[allow(dead_code)]
pub fn length<T>(list: &List<T>) -> usize {
    match list {
        List::Nil => 0,
        List::Cons(_, cdr) => 1 + length(cdr),
    }
}

#[allow(dead_code)]
pub fn append<T>(list: List<T>, value: T) -> List<T> {
    match list {
        List::Nil => List::Cons(value, Box::new(List::Nil)),
        List::Cons(car, cdr) => List::Cons(car, Box::new(append(*cdr, value))),
    }
}

#[allow(dead_code)]
pub fn in_list<T: PartialEq + Clone>(list: &List<T>, value: T) -> bool {
    match list {
        List::Nil => false,
        List::Cons(car, cdr) => {
            if *car == value {
                true
            } else {
                in_list(cdr, value)
            }
        }
    }
}

#[allow(dead_code)]
pub fn find_index<T: PartialEq + Clone>(list: &List<T>, target: T) -> i32 {
    if in_list(list, target.clone()) {
        match list {
            List::Nil => -1,
            List::Cons(car, cdr) => {
                if *car == target {
                    0
                } else {
                    1 + find_index(cdr, target)
                }
            }
        }
    } else {
        -1
    }
}

#[allow(dead_code)]
pub fn nth_element<T: Clone>(list: &List<T>, count: usize) -> Option<T> {
    match list {
        List::Nil => None,
        List::Cons(car, cdr) => match count {
            0 => Some(car.clone()),
            _ => nth_element(cdr, count - 1),
        },
    }
}

#[allow(dead_code)]
pub fn get_index<T>(list: &List<T>, index: u32) -> Option<&List<T>> {
    match list {
        List::Nil => None,
        List::Cons(_, cdr) => match index {
            0 => Some(list),
            _ => get_index(cdr, index - 1),
        },
    }
}

#[allow(dead_code)]
pub fn count<T: PartialEq + Clone>(list: &List<T>, target: T) -> usize {
    match list {
        List::Nil => 0,
        List::Cons(car, cdr) => match *car == target {
            true => 1 + count(cdr, target),
            false => 0 + count(cdr, target),
        },
    }
}

#[allow(dead_code)]
pub fn subst<T: PartialEq + Clone>(list: &List<T>, new: T, old: T) -> List<T> {
    match list {
        List::Nil => List::Nil,
        List::Cons(car, cdr) => {
            if *car == old {
                List::Cons(new.clone(), Box::new(subst(cdr, new, old)))
            } else {
                List::Cons(car.clone(), Box::new(subst(cdr, new, old)))
            }
        }
    }
}

#[allow(dead_code)]
pub fn swapper<T: PartialEq + Clone>(list: &List<T>, sub_1: T, sub_2: T) -> List<T> {
    match list {
        List::Nil => List::Nil,
        List::Cons(car, cdr) => {
            if *car == sub_1 {
                List::Cons(sub_2.clone(), Box::new(swapper(cdr, sub_1, sub_2)))
            } else if *car == sub_2 {
                List::Cons(sub_1.clone(), Box::new(swapper(cdr, sub_1, sub_2)))
            } else {
                List::Cons(car.clone(), Box::new(swapper(cdr, sub_1, sub_2)))
            }
        }
    }
}

#[allow(dead_code)]
pub fn remove_first<T: PartialEq + Clone>(list: &List<T>, target: T) -> Option<List<T>> {
    if in_list(list, target.clone()) {
        match list {
            List::Nil => None,
            List::Cons(car, cdr) => {
                if *car == target {
                    Some(*cdr.clone())
                } else {
                    Some(List::Cons(
                        car.clone(),
                        Box::new(remove_first(cdr, target).unwrap()),
                    ))
                }
            }
        }
    } else {
        None
    }
}

#[allow(dead_code)]
pub fn remove_all<T: PartialEq + Clone>(list: List<T>, target: T) -> List<T> {
    match remove_first(&list, target.clone()) {
        None => list,
        Some(item) => remove_all(item, target),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swapper() {
        let list: List<u32> = List::Cons(
            1,
            Box::new(List::Cons(
                2,
                Box::new(List::Cons(
                    1,
                    Box::new(List::Cons(4, Box::new(List::Cons(1, Box::new(List::Nil))))),
                )),
            )),
        );

        let result = swapper(&list, 1, 2);
        assert_eq!(count(&result, 1), 1, "{:?}", &result);
        assert_eq!(count(&result, 2), 3);
    }

    #[test]
    fn test_count() {
        let list: List<u32> = List::Cons(
            1,
            Box::new(List::Cons(
                2,
                Box::new(List::Cons(
                    1,
                    Box::new(List::Cons(4, Box::new(List::Cons(1, Box::new(List::Nil))))),
                )),
            )),
        );

        assert_eq!(count(&list, 1), 3);
    }

    #[test]
    fn test_subst() {
        let mut list: List<u32> = List::Cons(
            1,
            Box::new(List::Cons(
                2,
                Box::new(List::Cons(
                    1,
                    Box::new(List::Cons(4, Box::new(List::Cons(1, Box::new(List::Nil))))),
                )),
            )),
        );

        list = subst(&list, 7, 1);
        assert_eq!(count(&list, 7), 3);
    }

    #[test]
    fn test_remove_all() {
        let mut list: List<u32> = List::Cons(
            1,
            Box::new(List::Cons(
                2,
                Box::new(List::Cons(
                    1,
                    Box::new(List::Cons(4, Box::new(List::Cons(1, Box::new(List::Nil))))),
                )),
            )),
        );

        // Happy path
        list = remove_all(list, 1);
        assert_eq!(length(&list), 2);

        // Less Happy path
        list = remove_all(list, 8);
        assert_eq!(length(&list), 2);
    }

    #[test]
    fn test_remove_first() {
        let list: List<u32> = List::Cons(
            1,
            Box::new(List::Cons(
                2,
                Box::new(List::Cons(
                    1,
                    Box::new(List::Cons(4, Box::new(List::Cons(1, Box::new(List::Nil))))),
                )),
            )),
        );

        let result = remove_first(&list, 4).unwrap();
        assert_eq!(length(&result), 4, "{:?}", result);

        let result = remove_first(&list, 1).unwrap();
        assert_eq!(length(&result), 4, "{:?}", result);

        let result = remove_first(&list, 8);
        assert_eq!(result.is_none(), true);
    }

    #[test]
    fn test_get_index() {
        let list: List<u32> = List::Cons(
            1,
            Box::new(List::Cons(
                2,
                Box::new(List::Cons(
                    3,
                    Box::new(List::Cons(4, Box::new(List::Cons(5, Box::new(List::Nil))))),
                )),
            )),
        );

        assert_eq!(get_index(&list, 0).unwrap().car(), Some(1 as u32));
        assert_eq!(get_index(&list, 4).unwrap().car(), Some(5 as u32));
        assert_eq!(get_index(&list, 8).is_none(), true);
    }

    #[test]
    fn test_nth_element() {
        let list: List<u32> = List::Cons(
            1,
            Box::new(List::Cons(
                2,
                Box::new(List::Cons(
                    3,
                    Box::new(List::Cons(4, Box::new(List::Cons(5, Box::new(List::Nil))))),
                )),
            )),
        );

        assert_eq!(nth_element(&list, 0), Some(1 as u32));
        assert_eq!(nth_element(&list, 4), Some(5 as u32));
        assert_eq!(nth_element(&list, 20), None);
    }

    #[test]
    fn test_find_index() {
        let list: List<u32> = List::Cons(
            1,
            Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
        );

        assert_eq!(find_index(&list, 1), 0);
        assert_eq!(find_index(&list, 3), 2, "{:?}", &list);
        assert_eq!(find_index(&list, 6), -1, "{:?}", &list);
    }

    #[test]
    fn test_in_list() {
        let list: List<u32> = List::Cons(
            1,
            Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
        );

        assert_eq!(in_list(&list, 1), true);
        assert_eq!(in_list(&list, 3), true);
        assert_eq!(in_list(&list, 6), false);
    }

    #[test]
    fn test_append() {
        let mut list: List<u32> = List::Nil;

        list = append(list, 1);
        list = append(list, 2);
        list = append(list, 3);

        assert_eq!(length(&list), 3, "{:?}", &list);
    }

    #[test]
    fn test_length() {
        let list: List<i32> = List::Cons(
            1,
            Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
        );

        assert_eq!(length(&list), 3);
    }
}
