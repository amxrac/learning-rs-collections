use std::collections::HashMap;

fn main() {
    let mut v = vec![2,4,2,1,4,5,6,3,5,6,7,78,8,42,2,8,9,11,6,46,35,27,36,7,8,3,6,4,6,6,6,6,6];
    calc(&mut v);
}

fn calc(nums: &mut Vec<i32>) {
    nums.sort();

    println!("{:?}", nums);

    let len = nums.len();

    let median = if len % 2 == 0 {
        (nums[len/2 - 1] + nums[len/2]) as f64 / 2.0
    } else {
        nums[len/2] as f64
    };

    println!("median: {:?}", median);

    let mut frequency: HashMap<i32, usize> = HashMap::new();

    for num in nums {
        *frequency.entry(*num).or_insert(0) += 1;
    }

    let max_frequency = frequency.values().max().unwrap_or(&0);

    let mode: Vec<i32> = frequency.iter().filter(|&(_, &count)| count == *max_frequency).map(|(&key, _)| key).collect();

    println!("mode: {:?}", &mode);
}