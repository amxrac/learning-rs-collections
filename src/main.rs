use std::collections::HashMap;
use std::io;

fn main() {
    let mut v = vec![2,4,2,1,4,5,6,3,5,6,7,78,8,42,2,8,9,11,6,46,35,27,36,7,8,3,6,4,6,6,6,6,6];
    calc(&mut v);

    let w = "there exists infinite possibilities happening simultaneously";
    pig_latin(w);
    text_interface();
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
    let words: Vec<&str> = s.split_whitespace().collect();
    let mut modified_words: Vec<String> = Vec::new();

    for word in &words {
        match word.chars().next() {
            Some(c) if "aeiou".contains(c) => {
                let mut new_word = word.to_string();
                    new_word.push_str("hay");
                    modified_words.push(new_word);
            }
            Some(c) if "bcdfghjklmnpqrstvwxyz".contains(c) => {
                let first_char = word.chars().next().unwrap();
                let rest = &word[first_char.len_utf8()..];
                let result = format!("{}{}{}", rest, first_char, "ay");
                modified_words.push(result);
            }
            Some(_) => {
                modified_words.push(word.to_string());
            }
            None => {}
        }
    }
    println!("{:?}", modified_words);
}

fn text_interface() {
    let mut emp_hash: HashMap<String, Vec<String>> = HashMap::new();
    let mut employee_name = loop {
        println!("enter employee name");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read line");

        let name = input.trim().to_string();
        if !name.is_empty() {
            break name.to_string();
        }
        println!("enter a valid employee name");
    };

    let mut department_name = loop {
        println!("enter department name");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read line");

        let dept = input.trim().to_string();
        if !dept.is_empty() {
            break dept.to_string();
        }
        println!("enter a valid department name");
    };

    emp_hash.entry(department_name).or_insert(Vec::new()).push(employee_name);

    println!("enter 1 for employees by department, 2 for company employees");
    let mut query: i32 = loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read line");

        let input: i32 = match input.trim().parse() {
            Ok(num) if num == 1 || num == 2 => break num,
            Ok(_) => {
                println!("enter a valid input. 1 for employees in a department, 2 for all employees");
                continue;
            },
            Err(_) => {
                println!("enter a valid input. 1 for employees in a department, 2 for all employees");
                continue;
            }
        };
    };

    if query == 1 {
        println!("enter department name");
        let mut dept = loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("failed to read line");

            let dep = input.trim().to_string();

            if emp_hash.contains_key(&dep) {
                break dep.to_string();
            }
            println!("enter a valid department name");
        };

        match emp_hash.get(&dept) {
            Some(values) => println!("{:?}", values.join(" ")),
            None => println!("no employees found for {:?}", &dept),
        }
    }
    else {
        println!("{:?}",  emp_hash);
    }
}
