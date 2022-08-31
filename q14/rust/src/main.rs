use std::collections::HashMap;
use std::time::Instant;
fn main() {
    // if n is even n/2
    // if n is odd 3*n + 1
    // Count the length of the chain
    let max_attempts = 1000000;
    let attempts: Vec<u64> = (1..max_attempts).collect();
    collatz_no_cache(&attempts);
    hash_cache(&attempts, 1000000);
    vec_cache(&attempts, 1000000);
}

fn vec_cache(attempts: &Vec<u64>, capacity: usize) {
    let mut max_iterations = 0;
    let mut max_val: u64 = 0;
    let mut cache: Vec<u64> = vec![0; capacity];
    let now = Instant::now();
    for att in attempts {
        let iter = collatz_vec(*att, &cache);
        cache[*att as usize] = iter;
        if iter > max_iterations {
            max_val = *att;
            max_iterations = iter;
        }
    }
    let end = Instant::now();
    let total = end - now;
    println!(
        "Vec Max val {}, max iterations {} in time {:?}",
        max_val, max_iterations, total
    );
}

fn collatz_vec(num: u64, cache: &Vec<u64>) -> u64 {
    let mut iterations = 1;
    let mut n: u64 = num.into();
    while n != 1 {
        if let Some(cached_len) = cache.get(n as usize) {
            if cached_len != &0 {
                return iterations + cached_len - 1;
            }
        }
        if n % 2 == 0 {
            n = n / 2;
            iterations += 1;
        } else {
            n = 3 * n + 1;
            iterations += 1;
        }
    }
    return iterations;
}

fn hash_cache(attempts: &Vec<u64>, capacity: usize) {
    let mut max_iterations = 0;
    let mut max_val: u64 = 0;
    let mut cache: HashMap<u64, u64> = HashMap::with_capacity(capacity);

    let now = Instant::now();
    for att in attempts {
        let iter = collatz_hash(*att, &cache);
        cache.insert(*att, iter);
        if iter > max_iterations {
            max_val = *att;
            max_iterations = iter;
        }
    }
    let end = Instant::now();
    let total = end - now;
    println!(
        "Hash Max val {}, max iterations {}, in time {:?}",
        max_val, max_iterations, total
    );
}

fn collatz_hash(num: u64, cache: &HashMap<u64, u64>) -> u64 {
    let mut iterations = 1;
    let mut n: u64 = num.into();
    while n != 1 {
        if let Some(cached_len) = cache.get(&n) {
            return iterations + cached_len - 1;
        }
        if n % 2 == 0 {
            n = n / 2;
            iterations += 1;
        } else {
            n = 3 * n + 1;
            iterations += 1;
        }
    }
    return iterations;
}

fn collatz_no_cache(attempts: &Vec<u64>) {
    let now = Instant::now();
    let mut max_iterations = 0;
    let mut max_val = 0;
    for att in attempts {
        let iter = collatz(*att);
        if iter > max_iterations {
            max_val = *att;
            max_iterations = iter;
        }
    }
    let end = Instant::now();
    let total = end - now;
    println!(
        "Max val {}, with iterations {}, with time {:?}",
        max_val, max_iterations, total
    );
}

fn collatz(num: u64) -> u64 {
    let mut iterations = 1;
    let mut n: u64 = num.into();
    while n != 1 {
        if n % 2 == 0 {
            n = n / 2;
            iterations += 1;
        } else {
            n = 3 * n + 1;
            iterations += 1;
        }
    }
    return iterations;
}
