use sha256::digest;
use unknown_order::BigNumber;

// USE SHA256 HASH FUNCTION INSTEAD

pub fn hash_to_number(input: &str) -> BigNumber {
    BigNumber::from_slice(digest(input))
}

// MPZ has a next prime
// Function to find the next prime number greater than or equal to a given number
pub fn next_prime(start: BigNumber) -> BigNumber {
    let mut candidate = start;
    while !candidate.is_prime() {
        candidate = candidate + BigNumber::from(1);
    }
    candidate
}

// 3 !!!!!      -----     (base?)

// no exclude then?

// Hash a string to a prime number
pub fn map_string_to_prime(input: &str) -> BigNumber {
    let hash_value = hash_to_number(input);
    let prime_candidate = next_prime(hash_value);

    // // Ensure the prime is not equal to the excluded prime
    // if prime_candidate == exclude {
    //     return next_prime(prime_candidate + BigNumber::from(1));
    // }

    prime_candidate
}
