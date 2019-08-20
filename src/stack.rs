/// A stack implementation where each value is unique.

use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug)]
pub struct Stack<T> where T: PartialEq + Eq + Hash + Copy {
    values: Vec<(u8, T)>,
    generations: HashMap<T, u8>,
}

impl<T> Stack<T> where T: PartialEq + Eq + Hash + Copy {
    pub fn new() -> Stack<T> {
        Stack { values: Vec::new(), generations: HashMap::new() }
    }

    pub fn push(&mut self, value: T) {
        let generation = self.generations.entry(value).or_insert(0);
        *generation += 1;
        self.values.push((*generation, value));
    }

    pub fn pop(&mut self) -> Option<T> {
        loop {
            match self.values.pop() {
                Some((generation, value)) => {
                    let current_generation = self.generations.get_mut(&value).unwrap();
                    if generation == *current_generation {
                        return Some(value);
                    } else {
                        continue;
                    }
                }
                None => return None
            };
        }
    }

    pub fn insert_into(&mut self, stack: &mut Stack<T>) {
        let mut temp = Vec::new();
        while let Some(value) = self.pop() {
            temp.push(value);
        }

        while let Some(value) = temp.pop() {
            stack.push(value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_pop() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_insert_into() {
        let mut stack1 = Stack::new();
        stack1.push(1);
        stack1.push(2);
        stack1.push(3);

        let mut stack2 = Stack::new();
        stack2.push(1);
        stack2.push(2);
        stack2.push(3);

        stack1.insert_into(&mut stack2);
        assert_eq!(stack1.pop(), None);
        assert_eq!(stack2.pop(), Some(3));
        assert_eq!(stack2.pop(), Some(2));
        assert_eq!(stack2.pop(), Some(1));
        assert_eq!(stack2.pop(), None);
    }
}
