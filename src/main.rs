use std::time::Instant;
use clap::Parser;
use rand::Rng;

#[derive(Parser)]
struct Cli {
    size: i32
}

fn normal_solution(array: &Vec<i8>) -> f64 {
    let mut max = 0.0;
    for i in 1..array.len() + 1 {
        let mut sum = 0.0;
        for j in i..array.len() + 1 {
            sum += f64::from(array[j - 1]);
            if sum > max {
                max = sum
            }
        }
    }
    return max;
}

fn max_crossing_sum(array: &Vec<i8>, low: usize, mid: usize, high: usize) -> f64 {
    let mut sum: f64 = 0.0;
    let mut left_sum = f64::MIN;
    for i in (low..=mid).rev() {
        sum += f64::from(array[i]);
        if sum > left_sum {
            left_sum = sum;
        }
    }

    sum = 0.0;
    let mut right_sum = f64::MIN;
    for i in mid..=high {
        sum += f64::from(array[i]);
        if sum > right_sum {
            right_sum = sum;
        }
    }

    return (left_sum + right_sum - f64::from(array[mid]))
        .max(left_sum)
        .max(right_sum);
}

fn divide_and_conquer(array: &Vec<i8>, low: usize, high: usize) -> f64 {
    if low > high {
        return f64::MIN;
    }
    if low == high {
        return f64::from(array[low]);
    }

    let mid: usize = (low + high) / 2;
    let left_sum = divide_and_conquer(array, low, mid);
    let right_sum = divide_and_conquer(array, mid + 1, high);
    let cross_sum = max_crossing_sum(array, low, mid, high);

    return left_sum.max(right_sum).max(cross_sum);
}

fn kadane_solution(array: &Vec<i8>) -> f64 {
    let mut best_sum: f64 = f64::MIN;
    let mut current_sum: f64 = f64::MIN;
    for i in 0..array.len() {
        current_sum = f64::from(array[i]).max(current_sum + f64::from(array[i]));
        best_sum = best_sum.max(current_sum)
    }
    return best_sum;
}

fn generate_random_array(size: i32) -> Vec<i8> {
    let mut array = Vec::new();
    for _ in 0..size {
        array.push(rand::thread_rng().gen_range(-50..50));
    }
    return array;
}

fn main() {
    let args = Cli::parse();
    let array = generate_random_array(args.size);

    let start = Instant::now();
    let result = normal_solution(&array);
    let elapsed = start.elapsed();
    println!("\nResult Normal: {}", result);
    println!("Elapsed Time Normal: {:.2?}", elapsed);

    let start = Instant::now();
    let result = divide_and_conquer(&array, 0, array.len() - 1);
    let elapsed = start.elapsed();
    println!("\nResult Divide-and-conquer: {}", result);
    println!("Elapsed Time Divide-and-conquer: {:.2?}", elapsed);

    let start = Instant::now();
    let result = kadane_solution(&array);
    let elapsed = start.elapsed();
    println!("\nResult Kadane: {}", result);
    println!("Elapsed Time Kadane: {:.2?}", elapsed);
}
