use unknown_order::BigNumber;

// Function to generate primes up to a given limit, excluding a specific prime
pub fn generate_primes(limit: usize, exclude: BigNumber) -> Vec<BigNumber> {
    let mut primes = Vec::new();
    let mut is_prime = vec![true; limit + 1];

    for num in 2..=limit {
        if is_prime[num] {
            if BigNumber::from(num) != exclude {
                primes.push(BigNumber::from(num));
            }
            for multiple in (num * 2..=limit).step_by(num) {
                is_prime[multiple] = false;
            }
        }
    }
    primes
}

// 4 !!!!!

// Polynomial Rolling Hash Function

// Hash a string to a number
fn polynomial_hash(input: &str, base: u64, mod_val: u64) -> u64 {
    let mut hash: u64 = 0;
    for (i, byte) in input.bytes().enumerate() {
        hash = (hash + (byte as u64 * base.pow(i as u32)) % mod_val) % mod_val;
    }
    hash
}

// Hash a string to a prime number
pub fn map_string_to_prime(primes: &Vec<BigNumber>, input: &str) -> Option<BigNumber> {
    let mod_val = primes.len() as u64; // Use the number of primes for modulo
    let hash_value = polynomial_hash(input, 31, mod_val);
    let index = (hash_value % mod_val) as usize;
    Some(primes[index].clone())
}
