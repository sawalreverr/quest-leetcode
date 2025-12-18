fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
    return nums
        .iter()
        .cloned()
        .chain(nums.iter().map(|&x| x))
        .collect();
}

fn shuffle01(nums: Vec<i32>, n: i32) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let (first_half, second_half) = nums.split_at(n as usize);

    for i in 0..first_half.len() {
        result.push(first_half[i]);
        result.push(second_half[i]);
    }

    return result;
}

fn shuffle02(nums: Vec<i32>, n: i32) -> Vec<i32> {
    let (first_half, second_half) = nums.split_at(n as usize);

    first_half
        .iter()
        .zip(second_half)
        .flat_map(|(&x, &y)| [x, y])
        .collect()
}

fn find_max_consecutive_ones01(nums: Vec<i32>) -> i32 {
    let mut current: i32 = 0;
    let mut last: i32 = 0;

    for i in nums {
        if i == 1 {
            current += 1;
        } else {
            current = 0;
        }

        if last < current {
            last = current
        }
    }

    return last;
}

fn find_max_consecutive_ones02(nums: Vec<i32>) -> i32 {
    return nums.split(|&x| x == 0).map(|v| v.len()).max().unwrap_or(0) as i32;
}

fn main() {
    // Q1. Concatenation of array
    let nums: Vec<i32> = vec![1, 2, 3];
    let result = get_concatenation(nums);
    println!("{:?}", result);

    // Q2. Shuffle the array
    let nums: Vec<i32> = vec![2, 5, 1, 3, 4, 7];
    let result = shuffle02(nums, 3);
    println!("{:?}", result);

    // Q3. Max consecutive ones
    let nums: Vec<i32> = vec![1, 0, 1, 1, 0, 1];
    let result = find_max_consecutive_ones02(nums);
    println!("{:?}", result)
}
