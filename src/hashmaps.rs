struct MyHashMap {
    buckets: Vec<Vec<(String, i32)>>,
}

impl MyHashMap {
    pub fn new() -> Self {
        MyHashMap {
            buckets: (0..10).map(|_| Vec::new()).collect(),
        }
    }

    pub fn insert(&mut self, key: String, value: i32) {
        let index = key.len() % 10;

        let bucket = &mut self.buckets[index];

        for (k, v) in bucket.iter_mut() {
            if k == &key {
                *v = value;
                return;
            }
        }

        bucket.push((key, value));
    }

    pub fn get(&self, key: &str) -> Option<i32> {
        let index = key.len() % 10;

        let bucket = &self.buckets[index];

        for (k, v) in bucket {
            if k == key {
                return Some(*v);
            }
        }

        None
    }
}

pub fn use_hashmap() {
    let mut map = MyHashMap::new();

    map.insert("apple".to_string(), 5);
    map.insert("banana".to_string(), 10);

    println!("Running Test 1: Standard retrieval");
    assert_eq!(map.get("apple"), Some(5));
    assert_eq!(map.get("banana"), Some(10));
    println!("Test 1 passed");

    println!("Running Test 2: Updating value");
    map.insert("rust".to_string(), 1);
    // Insert the same key with a different value
    map.insert("rust".to_string(), 2);

    // The value should be updated, not duplicated
    assert_eq!(map.get("rust"), Some(2));
    println!("Test 2 passed");

    println!("Running Test 3: Retrieval from same bucket");
    map.insert("abc".to_string(), 1);
    map.insert("def".to_string(), 2);
    map.insert("ghi".to_string(), 3);

    // Verify we can still retrieve all of them despite being in the same bucket
    assert_for_all_elements(
        &map,
        vec![("abc", Some(1)), ("def", Some(2)), ("ghi", Some(3))],
    );
    println!("Test 3 passed");

    println!("Running Test 4: Retrieving non existent value");
    map.insert("exists".to_string(), 100);

    assert_eq!(map.get("does_not_exist"), None);
    println!("Test 4 passed");

    assert_eq!(map.get("anything"), None);

    // Helper function to clean up the test code
    fn assert_for_all_elements(map: &MyHashMap, cases: Vec<(&str, Option<i32>)>) {
        for (key, expected) in cases {
            assert_eq!(map.get(key), expected, "Failed for key: {}", key);
        }
    }
}
