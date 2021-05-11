pub fn nth(n: u32) -> u32 {
    let mut primes = vec![2];

    let mut counter = 3;
    while primes.len() <= n as usize {
        let mut is_prime = true;
        for prime in primes.iter() {
            if counter % prime == 0 {
                is_prime = false;
                break;
            }
        }

        if is_prime {
            primes.push(counter);
        }

        counter += 1;
    }

    *primes.last().unwrap()
}
