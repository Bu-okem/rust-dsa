use std::collections::VecDeque;

struct MyStack {
    data: Vec<char>,
}

struct MyQueue {
    data: VecDeque<i32>,
}

impl MyStack {
    fn new() -> Self {
        MyStack { data: Vec::new() }
    }

    fn push(&mut self, value: char) {
        self.data.push(value);
    }

    fn pop(&mut self) -> Option<char> {
        let value = self.data.pop();
        return value;
    }
}

impl MyQueue {
    fn new() -> Self {
        MyQueue {
            data: VecDeque::new(),
        }
    }
    fn enqueue(&mut self, value: i32) {
        self.data.push_back(value);
    }

    fn dequeue(&mut self) -> Option<i32> {
        self.data.pop_front()
    }
}

fn is_valid(s: &str) -> bool {
    let mut stack = MyStack::new();

    let mut encountered_bracket = false;

    if s.len() == 0 {
        return false;
    }

    for item in s.chars() {
        match item {
            '(' | '[' | '{' => {
                encountered_bracket = true;
                stack.push(item)
            }
            ')' | ']' | '}' => {
                encountered_bracket = true;
                let popped = stack.pop();
                if let Some(ch) = popped {
                    if matches!((ch, item), ('(', ')') | ('[', ']') | ('{', '}')) {
                        continue;
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            _ => {}
        }
    }

    return stack.data.is_empty() && encountered_bracket;
}

pub fn use_stacks_is_valid() {
    println!("Test 1: Empty string");
    assert_eq!(is_valid(""), false);
    println!("✅ Test 1 Passed!");

    println!("Test 2: Simple valid cases");
    assert_eq!(is_valid("()"), true);
    assert_eq!(is_valid("[]"), true);
    assert_eq!(is_valid("{}"), true);
    println!("✅ Test 2 Passed!");

    println!("Test 3: Complex nested validity");
    assert_eq!(is_valid("{[()]}"), true);
    assert_eq!(is_valid("((({[[[]]]})) )"), true);
    println!("✅ Test 3 Passed!");

    println!("Test 4: Mismatched types");
    assert_eq!(is_valid("(]"), false);
    assert_eq!(is_valid("[}"), false);
    assert_eq!(is_valid("{)"), false);
    println!("✅ Test 4 Passed!");

    println!("Test 5: Incorrect order interleaved");
    assert_eq!(is_valid("([)]"), false);
    assert_eq!(is_valid("{[(])}"), false);
    println!("✅ Test 5 Passed!");

    println!("Test 6: Unclosed brackets");
    assert_eq!(is_valid("("), false);
    assert_eq!(is_valid("((("), false);
    assert_eq!(is_valid("() ("), false);
    println!("✅ Test 6 Passed!");

    println!("Test 7: Extra closing brackets");
    assert_eq!(is_valid(")"), false);
    assert_eq!(is_valid("())"), false);
    assert_eq!(is_valid("[]}"), false);
    println!("✅ Test 7 Passed!");

    println!("Test 8: None bracket characters");
    assert_eq!(is_valid("(a[b]c)"), true);
    assert_eq!(is_valid("123!@#"), false);
    println!("✅ Test 8 Passed!");
}
