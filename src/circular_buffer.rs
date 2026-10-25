struct CircularBuffer {
    arr: [i32; 8],
    head: usize,
    tail: usize,
    len: usize,
}

impl CircularBuffer {
    pub fn new() -> Self {
        Self {
            arr: [0; 8],
            head: 0,
            tail: 0,
            len: 0,
        }
    }

    pub fn push(&mut self, value: i32) {
        self.arr[self.head] = value;
        self.head = (self.head + 1) % 8;

        if self.len < 8 {
            self.len += 1;
        } else {
            self.tail = (self.tail + 1) % 8;
        }
    }

    pub fn pop(&mut self) -> Option<i32> {
        if self.len == 0 {
            return None;
        }
        let value: i32 = self.arr[self.tail];
        self.tail = (self.tail + 1) % 8;
        self.len -= 1;

        Some(value)
    }
}

pub fn use_circular_buffer() {
    let mut cb = CircularBuffer::new();

    // Test 1: Basic Push/Pop
    cb.push(10);
    cb.push(20);
    assert_eq!(cb.pop(), Some(10));
    assert_eq!(cb.pop(), Some(20));
    assert_eq!(cb.pop(), None);

    // Test 2: Wrap Around logic
    for i in 0..7 {
        cb.push(i);
    } // Buffer now has [0, 1, 2, 3, 4, 5, 6]
    cb.push(7); // Buffer is now full [0, 1, 2, 3, 4, 5, 6, 7]
    cb.push(8); // Overwrites index 0. Tail should move to 1.

    assert_eq!(cb.pop(), Some(1)); // The '0' was overwritten by '8', so 1 is the new oldest
    assert_eq!(cb.pop(), Some(2));

    // Test 3: Emptying the buffer
    while cb.pop().is_some() {}
    assert_eq!(cb.pop(), None);

    println!("Day 1 Mastery Confirmed: All tests passed!");
}
