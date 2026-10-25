struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

struct LinkedList {
    head: Option<Box<Node>>,
}

impl LinkedList {
    fn new() -> Self {
        Self { head: None }
    }

    fn push_front(&mut self, value: i32) {
        let new_node: Node = Node {
            value,
            next: self.head.take(),
        };
        self.head = Some(Box::new(new_node));
    }

    fn pop_front(&mut self) -> Option<i32> {
        match self.head.take() {
            Some(node) => {
                let value = node.value;
                self.head = node.next;

                return Some(value);
            }
            None => None,
        }
    }
}

pub fn use_singly_linked_list() {
    let mut list = LinkedList::new();

    // --- TEST 1: The Empty State ---
    // A new list should immediately return None when popped.
    println!("Running Test 1: Empty State...");
    assert_eq!(list.pop_front(), None);
    assert_eq!(list.pop_front(), None);
    println!("✅ Test 1 Passed!");

    // --- TEST 2: LIFO (Last-In, First-Out) Sequence ---
    // If we push 10, then 20, then 30, the first one out MUST be 30.
    println!("\nRunning Test 2: LIFO Order...");
    list.push_front(10); // List: [10]
    list.push_front(20); // List: [20, 10]
    list.push_front(30); // List: [30, 20, 10]

    assert_eq!(list.pop_front(), Some(30));
    assert_eq!(list.pop_front(), Some(20));
    assert_eq!(list.pop_front(), Some(10));
    assert_eq!(list.pop_front(), None); // Should be empty now
    println!("✅ Test 2 Passed!");

    // --- TEST 3: Interleaved Stress Test ---
    // Testing the ability to move between empty and non-empty states
    // without breaking the pointer chain or causing a crash.
    println!("\nRunning Test 3: Interleaved Operations...");
    list.push_front(100);
    assert_eq!(list.pop_front(), Some(100)); // Empty again

    list.push_front(200);
    list.push_front(300);
    assert_eq!(list.pop_front(), Some(300));

    list.push_front(400);
    assert_eq!(list.pop_front(), Some(400));
    assert_eq!(list.pop_front(), Some(200));
    assert_eq!(list.pop_front(), None);

    println!("✅ Test 3 Passed!");
}
