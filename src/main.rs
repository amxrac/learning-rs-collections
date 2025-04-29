use std::collections::HashMap;

fn main() {
    // let mut v = vec![2,4,2,1,4,5,6,3,5,6,7,78,8,42,2,8,9,11,6,46,35,27,36,7,8,3,6,4,6,6,6,6,6];
    // calc(&mut v);

    let w = "there exists infinite possibilities happening simultaneously";
    pig_latin(w);

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

    let mode: Vec<i32> = frequency.iter()
                                   .filter(|&(_, &count)| count == *max_frequency)
                                   .map(|(&key, _)| key)
                                   .collect();

    println!("mode: {:?}", &mode);
}

fn pig_latin(s: &str) {
    let consonants = ['b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'w', 'x', 'y', 'z'];
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let words: Vec<&str> = s.split_whitespace().collect();
    let mut modified_words: Vec<String> = Vec::new();

    for word in &words {
        if consonants.contains(&word.chars().next().unwrap()) {
            let mut word_slice = String::from(word[1..].to_string());
            let mut new_word = String::from(word[0..1].to_string());


            word_slice.push_str(&new_word);
            word_slice.push_str("ay");
            modified_words.push(word_slice);
        }
        else if vowels.contains(&word.chars().next().unwrap()) {
            let mut new_word = String::from(word.to_string());
            new_word.push_str("hay");
            modified_words.push(new_word);
        }
    }
    println!("{:?}", modified_words);
}