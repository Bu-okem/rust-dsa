use sha2::{Digest, Sha256};
use std::collections::HashMap;

fn hash_data(input: &str) -> String {
    let mut hasher = Sha256::new();

    hasher.update(input.as_bytes());

    let result = hasher.finalize();
    format!("{:x}", result)
}

struct StateDatabase {
    storage: HashMap<String, String>,
}

impl StateDatabase {
    pub fn new() -> Self {
        StateDatabase {
            storage: HashMap::new(),
        }
    }

    pub fn insert(&mut self, user: String, balance: String) {
        self.storage.insert(user, balance);
    }

    pub fn get(&self, user: &str) -> Option<String> {
        self.storage.get(user).cloned()
    }

    pub fn commit(&self) -> String {
        let mut data: Vec<(&String, &String)> = self.storage.iter().collect();
        data.sort();

        let mut hashed: Vec<String> = Vec::new();

        for item in &data {
            hashed.push(hash_data(&format!("{}{}", item.0, item.1)));
        }

        while hashed.len() > 1 {
            let mut next_level = Vec::new();
            for i in (0..hashed.len()).step_by(2) {
                let left_child = &hashed[i];

                let mut right_child = left_child;

                if i + 1 < hashed.len() {
                    right_child = &hashed[i + 1];
                }

                next_level.push(hash_data(&format!("{left_child}{right_child}")));
            }
            hashed = next_level;
        }

        hashed[0].clone()
    }
}

pub fn use_db() {
    let mut db = StateDatabase::new();

    // 1. Insert initial data
    db.insert("user_1".to_string(), "balance_100".to_string());
    db.insert("user_2".to_string(), "balance_50".to_string());

    let root_v1 = db.commit();
    println!("Root V1: {}", root_v1);

    // 2. Update data (The state has changed!)
    db.insert("user_1".to_string(), "balance_150".to_string());

    let root_v2 = db.commit();
    println!("Root V2: {}", root_v2);

    // 3. The Verification
    if root_v1 != root_v2 {
        println!("✅ Success! The Root Hash changed after the update.");
    } else {
        println!("❌ Failure! The Root Hash did not change!");
    }
}
