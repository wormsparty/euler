// https://projecteuler.net/problem=23

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

fn is_sum_of_2_abundant(x: u64, abundants: &Vec<u64>) -> bool {
    for left_index in 0..abundants.len() {
        let left = abundants[left_index];

        if left > x {
            break
        }

        for right_index in left_index..abundants.len() {
            let right = abundants[right_index];
            let sum = left + right;

            if sum == x {
                return true;
            }

            if sum > x {
                break;
            }
        }
    }

    false
}

fn main() {
    let mut abundant_numbers = Vec::new();

    for i in 12..28124 {
        if is_abundant(i) {
            abundant_numbers.push(i);
        }
    }

    let mut sum = 0;

    for i in 24..28124 {
        if !is_sum_of_2_abundant(i, &abundant_numbers) {
            println!("{} is not a sum", i);
            sum += i;
        }
    }

    println!("sum: {}", sum);
}
