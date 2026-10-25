fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len() - 1;

    while low <= high {
        let mid = low + ((high - low) / 2);

        if arr[mid] == target {
            return Some(mid);
        } else if arr[mid] < target {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    None
}

fn max_window_sum(arr: &[i32], k: usize) -> i32 {
    let mut current_sum = 0;
    for i in 0..k {
        current_sum += arr[i];
    }

    let mut max_sum = current_sum;

    for i in k..arr.len() {
        current_sum = current_sum - arr[i - k] + arr[i];

        if current_sum > max_sum {
            max_sum = current_sum;
        }
    }

    max_sum
}
