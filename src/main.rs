// https://projecteuler.net/problem=23

use std::collections::HashMap;
use std::hash::Hash;

fn sum_of_divisors(x: u64) -> u64 {
    let mut divisor_sum = 0;

    for i in 1..x {
        if x % i == 0 {
            divisor_sum += i;
        }
    }

    divisor_sum
}

fn is_abundant(x: u64) -> bool {
    sum_of_divisors(x) > x
}

fn is_sum_of_2_abundant(x: u64, list: &Vec<u64>, hash: &HashMap<u64, u64>) -> bool {
    for left_index in 0..list.len() {
        let left = list[left_index];

        if left > x {
            break
        }

        let diff = x - left;
        
        if hash.contains_key(&diff) {
            return true
        }
    }

    false
}

fn main() {
    let mut abundant_numbers_hash = HashMap::new();
    let mut abundant_numbers_list = Vec::new();

    for i in 12..28124 {
        if is_abundant(i) {
            abundant_numbers_list.push(i);
            abundant_numbers_hash.insert(i, i);
        }
    }

    let mut sum = 0;

    for i in 24..28124 {
        if !is_sum_of_2_abundant(i, &abundant_numbers_list, &abundant_numbers_hash) {
            println!("{} is not a sum", i);
            sum += i;
        }
    }

    println!("sum: {}", sum);
}
