use std::collections::HashMap;

use primes;

fn main() {
    let mut divisors: HashMap<u64,u32> = HashMap::new();
    let mut pp = primes::primes();

    for i in 1..=20 {
        let mut cur_p = 0;
        let mut cur_count = 0;

        for p in pp.prime_factors(i) {
            if p != cur_p {
                if cur_count > 0 && cur_count > *divisors.get(&cur_p).unwrap_or(&0) {
                    divisors.insert(cur_p, cur_count);
                }
                cur_p = p;
                cur_count = 1;
            } else {
                cur_count += 1;
            }
        }
        if cur_count > 0 && cur_count > *divisors.get(&cur_p).unwrap_or(&0) {
            divisors.insert(cur_p, cur_count);
        }
    }

    let mut res: u64 = 1;

    for (k, v) in divisors {
        println!("{}: {}", k, v);
        res *= k.pow(v);
    }

    println!("{}", res);
}
