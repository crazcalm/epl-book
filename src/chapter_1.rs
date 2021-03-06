use crate::cons;

#[allow(dead_code)]
pub fn product<T: Clone>(list_1: &[T], list_2: &[T]) -> Option<Vec<Vec<T>>> {
    match list_1.is_empty() || list_2.is_empty() {
        true => None,
        false => match list_1.is_empty() {
            true => None,
            false => {
                let mut result: Vec<Vec<T>> = Vec::new();

                if let Some(item) = span(&list_2[..], list_1[0].clone()) {
                    result.extend_from_slice(&item);
                }

                if let Some(item) = product(&list_1[1..], list_2) {
                    result.extend_from_slice(&item);
                }

                Some(result)
            }
        },
    }
}

#[allow(dead_code)]
pub fn span<T: Clone>(list: &[T], value: T) -> Option<Vec<Vec<T>>> {
    match list.is_empty() {
        true => None,
        false => {
            let mut result = vec![vec![value.clone(), list[0].clone()]];

            if let Some(item) = span(&list[1..], value.clone()) {
                result.extend_from_slice(&item);
            }
            Some(result)
        }
    }
}

#[allow(dead_code)]
pub fn count_occurrences<T: PartialEq + Clone>(list: &[cons::List<T>], target: T) -> usize {
    match list.is_empty() {
        true => 0,
        false => match cons::in_list(&list[0], target.clone()) {
            true => {
                cons::count(&list[0], target.clone())
                    + count_occurrences(&list[1..], target.clone())
            }
            false => 0 + count_occurrences(&list[1..], target.clone()),
        },
    }
}

#[allow(dead_code)]
pub fn list_set<T: Clone>(
    list_1: &[cons::List<T>],
    index: usize,
    list_2: cons::List<T>,
) -> Vec<cons::List<T>> {
    if list_1.is_empty() {
        vec![]
    } else {
        match index {
            0 => {
                let mut result = vec![list_2];

                match list_1.len() {
                    0 | 1 => result,
                    _ => {
                        result.extend_from_slice(&list_1[1..]);
                        result
                    }
                }
            }
            _ => {
                let mut result = vec![list_1[0].clone()];
                result.extend_from_slice(&list_set(&list_1[1..], index - 1, list_2));
                result
            }
        }
    }
}

#[allow(dead_code)]
pub fn inverse<T: Clone>(list: &[&[T]]) -> Vec<Vec<T>> {
    match list.len() {
        0 => vec![vec![]],
        1 => vec![reverse(list[0])],
        _ => {
            let mut result = vec![reverse(list[0].clone())];
            result.extend_from_slice(&inverse(&list[1..]));
            result
        }
    }
}

#[allow(dead_code)]
pub fn down<T: Clone>(list: &[T]) -> Vec<Vec<T>> {
    match list.len() {
        0 => panic!("List cannot be empty"),
        1 => vec![vec![list[0].clone()]],
        _ => {
            let mut result = vec![vec![list[0].clone()]];
            result.extend_from_slice(&down(&list[1..]));
            result
        }
    }
}

#[allow(dead_code)]
pub fn reverse<T: Clone>(list: &[T]) -> Vec<T> {
    match list.len() {
        0 => vec![],
        1 => vec![list[0].clone()],
        _ => {
            let mut result = vec![list[list.len() - 1].clone()];
            let index = list.len() - 1;
            result.extend_from_slice(&reverse(&list[..index]));
            result
        }
    }
}

#[allow(dead_code)]
pub fn duple<T: Clone>(list: &[T], num: u32) -> Vec<T> {
    match list.len() {
        0 => vec![],
        1 => copy(list[0].clone(), num),
        _ => {
            let mut result = vec![];
            result.extend_from_slice(&copy(list[0].clone(), num));
            result.extend_from_slice(&duple(&list[1..], num));
            result
        }
    }
}

#[allow(dead_code)]
pub fn copy<T: Clone>(item: T, num: u32) -> Vec<T> {
    match num {
        0 | 1 => {
            vec![item]
        }
        _ => {
            let mut result = vec![item.clone()];
            result.extend_from_slice(&copy(item, num - 1).clone());
            result
        }
    }
}

#[allow(dead_code)]
fn list_length<T>(args: &[T]) -> u32 {
    match args.is_empty() {
        true => 0,
        false => {
            let split = args.split_first().unwrap();
            1 + list_length(split.1)
        }
    }
}

#[allow(dead_code)]
fn find_index<T: PartialEq + Clone>(args: &[T], target: T) -> i32 {
    match in_list(args, target.clone()) {
        false => -1,
        true => {
            let split = args.split_first().unwrap();

            if split.0.clone() == target {
                0
            } else {
                1 + find_index(split.1, target)
            }
        }
    }
}

#[allow(dead_code)]
fn nth_element<T>(arg: &[T], count: u32) -> &T {
    if list_length(arg) - 1 < count {
        panic!("splice does not have {} elements", count)
    };

    match count {
        0 => {
            let split = arg.split_first().expect("out of bounds exception");
            split.0
        }
        _ => {
            let split = arg.split_first().unwrap();
            nth_element(split.1, count - 1)
        }
    }
}

#[allow(dead_code)]
fn remove_first<T: Clone + PartialEq>(arg: &[T], target: T) -> Vec<T> {
    let index = find_index(arg, target);
    if index == -1 {
        arg.to_vec()
    } else {
        [
            arg[..index as usize].to_owned(),
            arg[index as usize + 1..].to_owned(),
        ]
        .concat()
    }
}

#[allow(dead_code)]
fn remove_all(arg: &[u32], target: u32) -> Vec<u32> {
    match arg.to_vec() == remove_first(arg, target) {
        true => arg.to_vec(),
        false => {
            let tempt = remove_first(arg, target);
            remove_all(&tempt, target)
        }
    }
}

#[allow(dead_code)]
fn in_list<T: Clone + PartialEq>(arg: &[T], target: T) -> bool {
    match arg.is_empty() {
        true => false,
        false => {
            let split = arg.split_first().unwrap();

            if split.0.to_owned() == target {
                true
            } else {
                in_list(split.1, target)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cons;

    #[test]
    fn test_product() {
        let list_1 = vec![1, 2, 3];
        let list_2 = vec![4, 5, 6];

        let result = product(&list_1[..], &list_2[..]);

        assert_eq!(
            result,
            Some(vec![
                vec![1, 4],
                vec![1, 5],
                vec![1, 6],
                vec![2, 4],
                vec![2, 5],
                vec![2, 6],
                vec![3, 4],
                vec![3, 5],
                vec![3, 6]
            ])
        );
    }

    #[test]
    fn test_span() {
        let list = vec![1, 2, 3, 4];

        let result = span(&list[..], 5);

        assert_eq!(
            result,
            Some(vec![vec![5, 1], vec![5, 2], vec![5, 3], vec![5, 4]])
        );
    }

    #[test]
    fn test_count_occurrences() {
        let list_a = cons::List::Cons(
            "a",
            Box::new(cons::List::Cons("b", Box::new(cons::List::Nil))),
        );
        let list_b = cons::List::Cons("b", Box::new(cons::List::Nil));
        let list_c = cons::List::Cons(
            "c",
            Box::new(cons::List::Cons("b", Box::new(cons::List::Nil))),
        );
        let list_d = cons::List::Cons(
            "d",
            Box::new(cons::List::Cons(
                "e",
                Box::new(cons::List::Cons("d", Box::new(cons::List::Nil))),
            )),
        );

        let list = vec![list_a, list_b, list_c, list_d];

        assert_eq!(count_occurrences(&list[..], "b"), 3);
        assert_eq!(count_occurrences(&list[..], "e"), 1);
        assert_eq!(count_occurrences(&list[..], "d"), 2);
    }

    #[test]
    fn test_list_set() {
        let list_a = cons::List::Cons("a", Box::new(cons::List::Nil));
        let list_b = cons::List::Cons("b", Box::new(cons::List::Nil));
        let list_c = cons::List::Cons("c", Box::new(cons::List::Nil));
        let list_d = cons::List::Cons(
            "d",
            Box::new(cons::List::Cons("e", Box::new(cons::List::Nil))),
        );

        let list = vec![list_a, list_b, list_c];
        let result = list_set(&list, 1, list_d);

        assert_eq!(result[1].car(), Some("d"), "{:?}", &result);
        assert_eq!(result[2].car(), Some("c"));
    }

    #[test]
    fn test_down() {
        let list = vec![1, 2, 3];

        let result = down(&list[..]);
        assert_eq!(result, vec![vec![1], vec![2], vec![3]]);
    }

    #[test]
    fn test_inverse() {
        let one = vec![1, 2, 3];
        let two = vec![9];
        let three = vec![3, 4];

        let list = vec![one.as_slice(), two.as_slice(), three.as_slice()];

        let result = inverse(list.as_slice());
        assert_eq!(result, vec![vec![3, 2, 1], vec![9], vec![4, 3]]);
    }

    #[test]
    fn test_reverse() {
        let list = vec![1, 2, 3, 4, 5];

        let result = reverse(&list[..]);
        assert_eq!(result, vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_duple() {
        let list = vec![1, 2, 3, 4];

        let result = duple(&list[..], 2);
        assert_eq!(result, vec![1, 1, 2, 2, 3, 3, 4, 4]);
    }

    #[test]
    fn test_copy() {
        let list: cons::List<u32> = cons::List::Cons(1, Box::new(cons::List::Nil));

        let result = copy(list.clone(), 3);
        assert_eq!(result.len(), 3);

        let result2 = copy(list.clone(), 0);
        assert_eq!(result2.len(), 1);
    }

    #[test]
    fn test_find_index() {
        let list = vec![5, 4, 3, 2, 1];

        // Happy path
        assert_eq!(find_index(&list[..], 2), 3);

        // Less happy path
        assert_eq!(find_index(&list[..], 8), -1);
    }

    #[test]
    fn test_list_length() {
        let list = vec![1, 2, 3, 4, 5, 6, 7];

        // empty list
        let mut empty_list = vec![1];
        let _ = empty_list.pop();

        assert_eq!(list_length(&list[..]), 7);
        assert_eq!(list_length(&empty_list[..]), 0);
    }

    #[test]
    #[should_panic(expected = "splice does not have 8 elements")]
    fn test_nth_element_panic() {
        let list = vec![1, 2, 3];

        nth_element(&list[..], 8);
    }

    #[test]
    fn test_nth_element() {
        let list = vec![5, 4, 3, 2, 1, 0];

        // Happy path
        assert_eq!(nth_element(&list, 3), &list[3]);
    }

    #[test]
    fn test_remove_all() {
        let list = vec![1, 2, 3, 1, 4, 1];

        // Happy path
        assert_eq!(remove_all(&list[..], 1), vec![2, 3, 4]);
        assert_eq!(remove_all(&list[..], 2), vec![1, 3, 1, 4, 1]);

        // Less Happy path
        assert_eq!(remove_all(&list[..], 5), list);
    }

    #[test]
    fn test_remove_first() {
        let list = vec![1, 2, 3];

        // Happy path
        assert_eq!(remove_first(&list[..], 1), vec![2, 3]);
        assert_eq!(remove_first(&list[..], 2), vec![1, 3]);

        // Less happy path
        assert_eq!(remove_first(&list[..], 5), vec![1, 2, 3]);
    }

    #[test]
    fn test_in_list() {
        let list = vec![1, 2, 3];

        // Happy path
        assert_eq!(in_list(&list[..], 1), true);
        assert_eq!(in_list(&list[..], 2), true);
        assert_eq!(in_list(&list[..], 3), true);

        // less happy path
        assert_eq!(in_list(&list[..], 0), false);
        assert_eq!(in_list(&list[..], 4), false);
    }
}
