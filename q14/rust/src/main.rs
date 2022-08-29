use std::collections::HashMap;
use std::time::Instant;
fn main() {
    // if n is even n/2
    // if n is odd 3*n + 1
    // Count the length of the chain
    //
    collatz_no_cache();
    hash_cache();
}

fn hash_cache() {
    let attempts: Vec<i32> = (1..1000000).collect();
    let now = Instant::now();
    let mut max_iterations = 0;
    let mut max_val = 0;
    let mut cache: HashMap<i64, i32> = HashMap::new();
    for att in attempts {
        let iter = collatz_hash(att, &cache);
        cache.insert(att.into(), iter); 
        if iter > max_iterations {
            max_val = att;
            max_iterations = iter;
        }
    }
    let end = Instant::now();
    let total = end - now; 
    println!("Hash Max val {}, max iterations {}, in time {:?}", max_val, max_iterations, total);

}

fn collatz_hash(num:i32, cache:&HashMap<i64, i32>) -> i32 {
    let mut iterations = 1;
    let mut n: i64 = num.into();
    while n != 1 {
        if cache.contains_key(&n){
            return iterations + cache.get(&n).unwrap() - 1; 
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

fn collatz_no_cache() {
    let attempts: Vec<i32> = (1..1000000).collect();
    let now = Instant::now();
    let mut max_iterations = 0;
    let mut max_val = 0;
    for att in attempts {
        let iter = collatz(att);
        if iter > max_iterations {
            max_val = att;
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

fn collatz(num: i32) -> i32 {
    let mut iterations = 1;
    let mut n: i64 = num.into();
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
