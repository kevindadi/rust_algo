pub fn sequential_search_pos(nums: &[i32], num: i32) -> Option<usize> {
    let mut pos: usize = 0;
    let mut found = false;
    let mut stop = false;

    while pos < nums.len() && !found  && !stop {
        if num == nums[pos] {
            found = true;
        } else if num < nums[pos] {
            stop = true; // 有序检测
        } else {
            pos += 1;
        }
    }

    if found {
        Some(pos)
    } else {
        None
    }
}