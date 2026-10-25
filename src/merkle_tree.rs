use sha2::{Digest, Sha256};

fn hash_data(input: &str) -> String {
    let mut hasher = Sha256::new();

    hasher.update(input.as_bytes());

    let result = hasher.finalize();
    format!("{:x}", result)
}

fn build_merkle_root(data: Vec<String>) -> String {
    let mut hashed: Vec<String> = Vec::new();

    for item in &data {
        hashed.push(hash_data(item));
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

pub fn use_merkle_tree() {
    println!(
        "{}",
        build_merkle_root(vec![
            "h8u47hfbw84".to_string(),
            "w8ub4f2g3rb".to_string(),
            "b82ger73782".to_string(),
            "uh82gr7g28g".to_string(),
            "bb8g747grf4".to_string(),
            "bd834g7gb72".to_string(),
            "bf474gf728f".to_string(),
        ])
    );
}
