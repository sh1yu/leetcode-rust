//155. 最小栈
//设计一个支持 push ，pop ，top 操作，并能在常数时间内检索到最小元素的栈。
//
// push(x) —— 将元素 x 推入栈中。
// pop()—— 删除栈顶的元素。
// top()—— 获取栈顶元素。
// getMin() —— 检索栈中的最小元素。
//
#[allow(dead_code)]
struct MinStack {
    cache: Vec<i32>,
    min: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl MinStack {
    /** initialize your data structure here. */
    fn new() -> Self {
        MinStack {
            cache: vec![],
            min: vec![],
        }
    }

    fn push(&mut self, val: i32) {
        self.cache.push(val);
        if self.min.is_empty() || val <= self.min[self.min.len() - 1] {
            self.min.push(val);
        }
    }

    fn pop(&mut self) -> i32 {
        let result = self.cache.pop().unwrap_or(-1);
        if self.min.len() > 0 && self.min[self.min.len() - 1] == result {
            self.min.pop();
        }
        result
    }

    fn top(&self) -> i32 {
        if self.cache.is_empty() {
            -1
        } else {
            self.cache[self.cache.len() - 1]
        }
    }

    fn get_min(&self) -> i32 {
        if self.min.is_empty() {
            -1
        } else {
            self.min[self.min.len() - 1]
        }
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut obj = MinStack::new();
        obj.push(3);
        obj.push(2);
        obj.push(5);
        obj.push(6);
        assert_eq!(6, obj.pop());
        assert_eq!(5, obj.top());
        assert_eq!(2, obj.get_min());
    }
}
