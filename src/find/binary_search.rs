pub fn binary_search1(nums: &[i32], num: i32) -> bool {
    let mut low = 0;
    let mut high = nums.len() - 1;
    let mut found = false;

    while low <= high && !found {
        let mid: usize = (low + high) >> 1;

        if num == nums[mid] {
            found = true;
        } else if num < nums[mid] {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }

    found
}

pub fn binary_search2(nums: &[i32], num: i32) -> bool {
    if 0 == nums.len() {return false;}

    let mid: usize = nums.len() >> 1;

    if num == nums[mid] {
        return true;
    } else if num < nums[mid] {
        return binary_search2(&nums[..mid], num);
    } else {
        return binary_search2(&nums[mid+1..], num);
    }
}