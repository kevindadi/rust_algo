pub fn nums_sum(nums: Vec<i32>) -> i32 {
    let mut sum = 0;
    for num in nums {
        sum += num;
    }

    sum
}

pub fn nums_sum3(sum: i32, nums: &[i32]) -> i32 {
    if 1 == nums.len() {
        sum + nums[0]
    } else {
        nums_sum3(sum + nums[0], &nums[1..])
    }
}
