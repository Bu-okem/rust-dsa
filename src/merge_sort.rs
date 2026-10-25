fn merge(left: &[i32], right: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    let mut left_pointer: usize = 0;
    let mut right_pointer: usize = 0;

    while left_pointer < left.len() && right_pointer < right.len() {
        if left[left_pointer] <= right[right_pointer] {
            result.push(left[left_pointer]);
            left_pointer += 1;
        } else {
            result.push(right[right_pointer]);
            right_pointer += 1;
        }
    }

    result.extend(&left[left_pointer..]);
    result.extend(&right[right_pointer..]);

    result
}

fn merge_sort(arr: &[i32]) -> Vec<i32> {
    if arr.len() == 1 {
        return Vec::from(arr);
    }

    let mid = arr.len() / 2;

    let left = &arr[..mid];
    let right = &arr[mid..];

    let sorted_left = merge_sort(left);
    let sorted_right = merge_sort(right);

    let final_vec = merge(&sorted_left, &sorted_right);

    final_vec
}
