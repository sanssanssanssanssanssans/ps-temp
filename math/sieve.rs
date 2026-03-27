enum SieveResult {
    Bool(Vec<bool>),
    List(Vec<usize>),
}

fn sieve(n: usize, mode: i32) -> SieveResult {
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    if n >= 1 {
        is_prime[1] = false;
    }
    let lim = (n as f64).sqrt() as usize;
    for i in 2..=lim {
        if is_prime[i] {
            let mut j = i * i;
            while j <= n {
                is_prime[j] = false;
                j += i;
            }
        }
    }
    if mode == 1 {
        let primes = is_prime.iter()
            .enumerate()
            .filter(|&(_, &p)| p)
            .map(|(i, _)| i)
            .collect();
        SieveResult::List(primes)
    } else {
        SieveResult::Bool(is_prime)
    }
}
