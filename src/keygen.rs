use crate::group_hash_function::map_string_to_prime;
use std::sync::Arc;
use unknown_order::*;

fn is_odd(bn: &BigNumber) -> bool {
    let bytes = bn.to_bytes();
    bytes.last().map_or(false, |&b| b % 2 != 0)
}

fn is_even(bn: &BigNumber) -> bool {
    !is_odd(bn)
}

pub fn jacobi(n: &BigNumber, k: &BigNumber) -> i32 {
    assert!(
        k > &BigNumber::from(0) && is_odd(k),
        "k must be positive and odd"
    );

    let mut n = n % k;
    let mut k = k.clone();
    let mut t = 1;

    while n != BigNumber::from(0) {
        while is_even(&n) {
            n /= BigNumber::from(2);
            let r = &k % BigNumber::from(8);
            if r == BigNumber::from(3) || r == BigNumber::from(5) {
                t = -t;
            }
        }
        std::mem::swap(&mut n, &mut k);
        if &n % BigNumber::from(4) == BigNumber::from(3)
            && &k % BigNumber::from(4) == BigNumber::from(3)
        {
            t = -t;
        }
        n %= &k;
    }

    if k == BigNumber::from(1) {
        t
    } else {
        0
    }
}

pub fn sample_element_with_jacobi(modulus: &BigNumber) -> BigNumber {
    loop {
        // Generate a random element g from the group G^a
        let g = BigNumber::random(&modulus);

        // Check the Jacobi symbol
        let jacobi_symbol = jacobi(&g, &modulus);
        if jacobi_symbol == 1 {
            return g;
        }
    }
}

// Add lambda parameter

/// Generates the public parameters and the initial commitment.
///
/// # Returns
///
/// A tuple containing the public parameters and the initial commitment.
pub fn keygen(
    lambda: usize,
) -> (
    (
        Group,
        BigNumber,
        Arc<dyn Fn(&str) -> BigNumber + Send + Sync>,
    ),
    (BigNumber, BigNumber),
) {
    // 1 !!!!!      ----->       size valid?    ---->     Function that maps lambda to size
    // Map lambda to bit size
    let bit_size = lambda_to_bit_size(lambda);

    // Create a safe group of unknown order
    let p = BigNumber::safe_prime(bit_size);
    let q = BigNumber::safe_prime(bit_size);

    // Calculate the modulus
    let modulus = p.clone() * q.clone();

    // Create the group
    let group = Group { modulus };

    // // Get the exponent from the lambda
    // let exp = BigNumber::from(lambda);

    // Create the hash function
    let hash_function: Arc<dyn Fn(&str) -> BigNumber + Send + Sync> =
        { Arc::new(move |input: &str| map_string_to_prime(input)) };

    let g = sample_element_with_jacobi(&group.modulus);

    // Create the public parameters
    let pp = (group, g.clone(), hash_function);
    let c = (BigNumber::from(1), g.clone());

    (pp, c)
}

/// Maps the security parameter lambda to a bit size for prime generation.
fn lambda_to_bit_size(_lambda: usize) -> usize {
    // Example mapping: directly use lambda as the bit size
    // You can adjust this logic based on your security requirements
    1024
}
