fn n_to_digits(n: u64) -> Vec<u8> {
    let mut curr_n = n;
    let mut res = vec![];

    while curr_n > 0 {
        res.push((curr_n % 10) as u8);
        curr_n = curr_n / 10;
    }

    res
}

fn is_palindrome(n: u64) -> bool {
    let digits = n_to_digits(n);

    digits.iter().eq(digits.iter().rev())
}

fn main() {
    let mut res = 0;

    for i in 100..1000 {
        for j in (i+1)..1000 {
            if is_palindrome(i*j) {
                if res < i*j {
                    res = i*j;
                }
            }
        }
    }

    println!("{}", res);
}
