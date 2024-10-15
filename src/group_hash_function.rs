use unknown_order::BigNumber;

// Polynomial Rolling Hash Function
fn polynomial_hash(input: &str, base: BigNumber, mod_val: &BigNumber) -> BigNumber {
    let mut hash = BigNumber::zero();
    for (i, byte) in input.bytes().enumerate() {
        let byte_value = BigNumber::from(byte);
        let base_pow = base.modpow(&BigNumber::from(i), &BigNumber::from(1));
        hash = (hash + (byte_value * base_pow) % mod_val) % mod_val;
    }
    hash
}

// Function to find the next prime number greater than or equal to a given number
fn next_prime(start: BigNumber) -> BigNumber {
    let mut candidate = start;
    while !candidate.is_prime() {
        candidate = candidate + BigNumber::from(1);
    }
    candidate
}

// 3 !!!!!      -----     (base?)

// Hash a string to a prime number
pub fn map_string_to_prime(limit: BigNumber, exclude: BigNumber, input: &str) -> BigNumber {
    // Base value of 31 is used for the polynomial hash function
    let base = BigNumber::from(31);
    let hash_value = polynomial_hash(input, base, &limit);
    let prime_candidate = next_prime(hash_value);

    // Ensure the prime is not equal to the excluded prime
    if prime_candidate == exclude {
        return next_prime(prime_candidate + BigNumber::from(1));
    }

    prime_candidate
}
