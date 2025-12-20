fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
    let mut found = vec![0; nums.len()];
    for i in 0..nums.len() {
        let idx = nums[i];
        found[idx as usize - 1] += 1;
    }

    return vec![2, 0]
        .iter()
        .filter_map(|&t| found.iter().position(|&x| x == t))
        .map(|i| (i + 1) as i32)
        .collect();
}

fn smaller_numbers_than_current01(nums: Vec<i32>) -> Vec<i32> {
    let mut ans = vec![0; nums.len()];

    for i in 0..nums.len() {
        for j in 0..nums.len() {
            if nums[j] < nums[i] {
                ans[i] += 1;
            }
        }
    }

    return ans;
}

fn smaller_numbers_than_current02(nums: Vec<i32>) -> Vec<i32> {
    return nums
        .iter()
        .map(|&x| nums.iter().filter(|&&y| y < x).count() as i32)
        .collect();
}

fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
    let mut found = vec![0; nums.len()];

    for i in 0..nums.len() {
        found[nums[i] as usize - 1] += 1;
    }

    return found
        .iter()
        .enumerate()
        .filter_map(|(i, &v)| (v == 0).then_some(i))
        .map(|i| (i + 1) as i32)
        .collect();
}

fn main() {
    // Q1. Set mismatch
    let nums: Vec<i32> = vec![3, 2, 2];
    let result = find_error_nums(nums);
    println!("{:?}", result);

    // Q2. How many number are smaller than the current number
    let nums: Vec<i32> = vec![8, 1, 2, 2, 3];
    let result = smaller_numbers_than_current02(nums);
    println!("{:?}", result);

    // Q3. Find all numbers disappeared in an array
    let nums: Vec<i32> = vec![4, 3, 2, 7, 8, 2, 3, 1];
    let result = find_disappeared_numbers(nums);
    println!("{:?}", result);
}
