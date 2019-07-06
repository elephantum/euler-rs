pub struct Primes {
    cache: Vec<u64>,
}

pub struct PrimesIter<'a> {
    curr: u64,
    primes: &'a mut Primes,
}

pub fn primes() -> Primes {
    Primes { cache: vec![] }
}

impl Primes {
    pub fn iter(&mut self) -> PrimesIter {
        PrimesIter { curr: 1, primes: self }
    }

    pub fn is_prime(&mut self, n: u64) -> bool {
        for p in self.iter() {
            if p >= n { break; }

            if n % p == 0 { return false; }
        }

        true
    }

    pub fn prime_factors(&mut self, n: u64) -> Vec<u64> {
        let mut curr_n = n;
        let mut res = vec![];

        for p in self.iter() {
            if p > curr_n { break; }

            while curr_n % p == 0 {
                res.push(p);
                curr_n = curr_n / p;
            }
        }

        res
    }
}

impl<'a> Iterator for PrimesIter<'a> {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        loop {
            self.curr += 1;

            let mut is_prime = true;

            for cached_prime in self.primes.cache.iter() {
                if self.curr <= *cached_prime {
                    break;
                }
                if self.curr % *cached_prime == 0 {
                    is_prime = false;
                    break;
                }
            }

            if is_prime {
                self.primes.cache.push(self.curr);
                return Some(self.curr);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_iter() {
        let mut p = super::primes();
        let mut pp = p.iter();

        assert_eq!(pp.next(), Some(2));
        assert_eq!(pp.next(), Some(3));
        assert_eq!(pp.next(), Some(5));
        assert_eq!(pp.next(), Some(7));
        assert_eq!(pp.next(), Some(11));
    }

    #[test]
    fn test_is_prime() {
        let mut pp = super::primes();

        assert!(pp.is_prime(2));
        assert!(pp.is_prime(3));
        assert!(! pp.is_prime(4));
        assert!(pp.is_prime(5));
        assert!(! pp.is_prime(6));
        assert!(pp.is_prime(7));

        assert!(pp.is_prime(17));
    }

    #[test]
    fn test_prime_factors() {
        let mut pp = super::primes();

        assert_eq!(pp.prime_factors(6), vec![2, 3]);
        assert_eq!(pp.prime_factors(15), vec![3, 5]);
        assert_eq!(pp.prime_factors(12), vec![2, 2, 3]);
    }
}
