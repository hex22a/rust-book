use std::collections::HashMap;

fn find_median(nums: &mut Vec<i32>) -> i32 {
    nums.sort();
    let middle = nums.len() / 2;
    nums[middle]
}

fn find_mode(nums: &Vec<i32>) -> i32 {
    let mut freq = HashMap::new();
    for n in nums {
        let count = freq.entry(*n).or_insert(0);
        *count += 1;
    }
    freq.iter().max_by_key(|(_, v)| *v).unwrap().0.to_owned()
}

pub fn run() {
    let mut nums = vec![5, 2, 1, 3, 4, 5, 4, 4, 4, 6];
    println!("median: {}", find_median(&mut nums));
    println!("mode: {}", find_mode(&nums));
}
