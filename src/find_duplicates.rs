use std::collections::HashSet;
use std::hash::Hash;
use std::time::Instant;
fn brute_force<T: PartialEq>(arr: &[T]) {
    let n = arr.len();
    for i in 0..n {
        for j in (i + 1)..n {
            if arr[i] == arr[j] {
                println!("Contains duplicate");
                return;
            }
        }
    }
    println!("No duplicates found");
}

fn sorting<T: PartialEq + Ord>(arr: &mut [T]) {
    arr.sort();
    let n = arr.len();
    for i in 0..n - 1 {
        if arr[i] == arr[i + 1] {
            println!("Contains duplicate");
            return;
        }
    }
    println!("No duplicates found");
}

fn hashing<T: Eq + Hash>(arr: &[T]) {
    let mut set = HashSet::new();

    for item in arr {
        if !set.insert(item) {
            println!("Contains duplicate");
            return;
        }
    }
    println!("No duplicates found");
}

pub fn find_duplicates() {
    let mut data: Vec<i32> = (0..100000).collect();
    data.push(99999);

    let start = Instant::now();
    brute_force(&data);
    let duration = start.elapsed();

    println!("Brute Force took: {:?}", duration);

    let start = Instant::now();
    hashing(&data);
    let duration = start.elapsed();

    println!("Hash Set took: {:?}", duration);

    let start = Instant::now();
    sorting(&mut data);
    let duration = start.elapsed();

    println!("Sorting took: {:?}", duration);
}
