fn count(arg: &[u32]) {
    println!("{:#?}", arg);

    if let Some((_, elements)) = arg.split_first() {
        println!("{:#?}", elements);
    }
}

fn list_length<T>(args: &[T]) -> u32 {
    match args.is_empty() {
        true => 0,
        false => {
            let split = args.split_first().unwrap();
            1 + list_length(split.1)
        }
    }
}

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

fn remove_all(arg: &[u32], target: u32) -> Vec<u32> {
    match arg.to_vec() == remove_first(arg, target) {
        true => arg.to_vec(),
        false => {
            let tempt = remove_first(arg, target);
            remove_all(&tempt, target)
        }
    }
}

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

//TODO: convert this to unit tests
fn main() {
    let test = vec![1, 2, 3, 4, 5, 4, 2, 4, 6];
    println!("{}", list_length(&test[..]));

    println!("{}", nth_element(&test[..], 3));

    println!("{:?}", in_list(&test[..], 4));

    println!("find_index {:?}", find_index(&test[..], 16));

    println!("remove_first {:?}", remove_first(&test[..], 4));
    println!("{:?}", remove_all(&test[..], 4));
    println!("{}", vec![1, 2] == vec![1, 2])
}
