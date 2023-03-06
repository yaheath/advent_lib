pub fn firstfac(x: u64) -> u64 {
    if x % 2 == 0 {
        return 2;
    };
     for n in (3..).step_by(2).take_while(|m| m*m <= x) {
        if x % n == 0 {
            return n;
        };
    }
    x
}

pub fn is_prime(x: u64) -> bool {
    match x {
        0 | 1 => false,
        2 => true,
        n => firstfac(n) == n,
    }
}

pub fn prime_factors(x: u64) -> Vec<u64> {
    if x <= 1 {
        return vec![];
    };
    let mut lst: Vec<u64> = Vec::new();
    let mut current = x;
    loop {
        let m = firstfac(current);
        lst.push(m);
        if current == m {
            break;
        }
        while current % m == 0 {
            current /= m;
        }
        if current == 1 {
            break;
        }
    }
    lst
}
