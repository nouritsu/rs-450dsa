// Problem 0 from https://450dsa.com/stacks_queues
#[derive(Debug)]
pub struct Stack<T>
where
    T: Default + Clone,
{
    arr: Vec<T>,
    ptr: usize,
}

impl<T> Stack<T>
where
    T: Default + Clone,
{
    pub fn new(init_size: usize) -> Self {
        Self {
            arr: vec![T::default(); init_size],
            ptr: 0,
        }
    }

    pub fn push(&mut self, x: T) -> bool {
        if self.ptr == self.arr.capacity() {
            return false;
        }

        self.arr[self.ptr] = x;
        self.ptr += 1;
        true
    }

    pub fn pop(&mut self) -> Option<&T> {
        if self.ptr == 0 {
            return None;
        }

        self.ptr -= 1;
        Some(&self.arr[self.ptr])
    }

    pub fn peek(&self) -> Option<&T> {
        if self.ptr == 0 {
            return None;
        }

        Some(&self.arr[self.ptr - 1])
    }

    pub fn len(&self) -> usize {
        self.ptr
    }

    pub fn capacity(&self) -> usize {
        self.arr.capacity()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn stack_push() {
        let mut stack = super::Stack::new(8);
        assert!(stack.push(1));
        assert!(stack.push(4));
        assert!(stack.push(9));
        assert!(stack.push(16));
        assert!(stack.push(25));
        assert!(stack.push(36));
        assert!(stack.push(49));
        assert!(stack.push(64));
        assert!(!stack.push(81));
    }

    #[test]
    fn stack_pop() {
        let mut stack = super::Stack::new(8);
        stack.push(1);
        stack.push(4);
        stack.push(9);
        stack.push(16);
        stack.push(25);

        assert_eq!(Some(&25), stack.pop());
        assert_eq!(Some(&16), stack.pop());
        assert_eq!(Some(&9), stack.pop());
        assert_eq!(Some(&4), stack.pop());
        assert_eq!(Some(&1), stack.pop());
        assert_eq!(None, stack.pop());
    }

    #[test]
    fn stack_peek() {
        let mut stack = super::Stack::new(8);
        stack.push(1);
        assert_eq!(Some(&1), stack.peek());
        stack.pop();
        assert_eq!(None, stack.peek());
    }

    #[test]
    fn stack_len() {
        let mut stack = super::Stack::new(8);
        assert_eq!(0, stack.len());
        stack.push(1);
        stack.push(4);
        stack.push(9);
        stack.push(16);
        stack.push(25);
        assert_eq!(5, stack.len());
    }

    #[test]
    fn stack_capacity() {
        let mut stack = super::Stack::new(8);
        assert_eq!(8, stack.capacity());
        stack.push(1);
        stack.push(4);
        stack.push(9);
        stack.push(16);
        stack.push(25);
        assert_eq!(8, stack.capacity());
    }
}
