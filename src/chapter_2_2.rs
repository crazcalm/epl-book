use std::fmt::Debug;

pub type Stack<T> = Vec<T>;

pub trait BasicStack<T> {
    fn empty_stack() -> Self;
    fn push(&mut self, item: T);
    fn pop(&mut self) -> Option<T>;
    fn peek(&mut self) -> Option<&T>;
    fn is_empty(&self) -> bool;
}

pub trait Env<T> {
    fn empty_env(&self) -> Self;
    fn extend_env(&self, key: String, value: T) -> Self;
    fn apply_env(&self, func: fn(T) -> T, target: String) -> Option<T>;
}

impl<T: Clone + Debug> Env<T> for Stack<(String, T)> {
    fn empty_env(&self) -> Self {
        Stack::empty_stack()
    }

    fn extend_env(&self, key: String, value: T) -> Self {
        let mut new_env: Stack<(String, T)> = Vec::with_capacity(self.capacity());

        for item in self.iter() {
            new_env.push(item.clone());
        }

        new_env.push((key, value));
        new_env
    }

    fn apply_env(&self, func: fn(T) -> T, target: String) -> Option<T> {
        let mut result = None;

        for (key, value) in self.iter() {
            if *key == target {
                result = Some(func(value.clone()));
            }
        }

        result
    }
}

impl<T> BasicStack<T> for Stack<T> {
    fn empty_stack() -> Self {
        vec![]
    }

    fn push(&mut self, item: T) {
        self.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.pop()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn peek(&mut self) -> Option<&T> {
        self.first()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_env_functions() {
        let env: Stack<(String, u32)> = Stack::empty_stack();

        let env_1 = env.extend_env("d".to_string(), 6);
        let env_2 = env_1.extend_env("y".to_string(), 8);
        let env_3 = env_2.extend_env("x".to_string(), 7);
        let env_4 = env_3.extend_env("y".to_string(), 14);

        assert_eq!(env, vec![]);
        assert_eq!(env_1, vec![("d".to_string(), 6)]);
        assert_eq!(env_2, vec![("d".to_string(), 6), ("y".to_string(), 8)]);
        assert_eq!(
            env_3,
            vec![
                ("d".to_string(), 6),
                ("y".to_string(), 8),
                ("x".to_string(), 7)
            ]
        );
        assert_eq!(
            env_4,
            vec![
                ("d".to_string(), 6),
                ("y".to_string(), 8),
                ("x".to_string(), 7),
                ("y".to_string(), 14)
            ]
        );

        assert_eq!(env_4.apply_env(|x| x * 2, "y".to_string()), Some(28 as u32));
    }
}
