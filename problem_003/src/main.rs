use primes;

fn main() {
    let mut pp = primes::primes();

    let factors = pp.prime_factors(600_851_475_143);

    println!("{}", factors.last().unwrap())
}
