struct MyQueue {
    stack_in: Vec<i32>,
    stack_out: Vec<i32>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {
    fn rotate(&mut self) {
        // if self.stack_out.is_empty() {
        //     for &x in self.stack_in.iter().rev() {
        //         self.stack_out.push(x);
        //     }
        //     self.stack_in.clear();
        // }
        // if self.stack_out.is_empty() {
        //     while !self.stack_in.is_empty() {
        //         self.stack_out.push(self.stack_in.pop().unwrap());
        //     }
        // }
        if self.stack_out.is_empty() {
            while let Some (x) = self.stack_in.pop() {
                self.stack_out.push(x);
            }
        }
    }

    fn new() -> Self {
        Self {
            stack_in: vec![],
            stack_out: vec![]
        }
    }

    fn push(&mut self, x: i32) {
        self.stack_in.push(x);
    }

    fn pop(&mut self) -> i32 {
        self.rotate();
        self.stack_out.pop().unwrap()
    }

    fn peek(&mut self) -> i32 {
        self.rotate();
        *self.stack_out.last().unwrap()
    }

    fn empty(&self) -> bool {
        self.stack_in.is_empty() && self.stack_out.is_empty()
    }
}

/**
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */

#[cfg(test)]
mod tests {
    use crate::MyQueue;

    #[test]
    fn example1() {
        let mut obj = MyQueue::new();
        obj.push(1);
        obj.push(2);
        assert_eq!(1, obj.peek());
        assert_eq!(1, obj.pop());
        assert!(!obj.empty());
    }
    #[test]
    fn failed1() {
        let mut obj = MyQueue::new();
        obj.push(1);
        assert_eq!(1, obj.pop());
        assert!(obj.empty());
    }
}

fn main() {
    println!("Hello, world!");
}
struct Solution;