use sha256::digest;
use unknown_order::BigNumber;

fn hash_to_number(input: &str) -> BigNumber {
    BigNumber::from_slice(digest(input))
}

pub fn next_prime(start: BigNumber) -> BigNumber {
    let two = BigNumber::from(2);
    let mut candidate = if start <= two {
        two.clone()
    } else if &start % &two == BigNumber::from(0) {
        start + BigNumber::from(1)
    } else {
        start
    };

    while !candidate.is_prime() {
        candidate = candidate + two.clone();
    }
    candidate
}

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
