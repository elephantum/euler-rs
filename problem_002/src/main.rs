struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;
    
    fn next(&mut self) -> Option<u32> {
        let new_next = self.curr + self.next;

        self.curr = self.next;
        self.next = new_next;

        Some(self.curr)
    }
}

fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 1, next: 1 }
}

fn main() {
    let mut sum: u64 = 0;

    for i in fibonacci() {
        if i > 4_000_000 { break; }
        // if i > 100 { break; }

        if i % 2 == 0 {
            sum += i as u64
        }
    }

    println!("{}", sum);
}