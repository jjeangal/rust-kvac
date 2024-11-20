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

fn jacobi(n: &BigNumber, k: &BigNumber) -> i32 {
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

// Key generation algorithm for Key Value Commitments (KVaC)
pub fn keygen(
    lambda: usize,
) -> (
    (
        // BigNumber,
        // BigNumber,
        Group,
        BigNumber,
        Arc<dyn Fn(&str) -> BigNumber + Send + Sync>,
    ),
    (BigNumber, BigNumber),
) {
    // 1 !!!!!      ----->       size valid?    ---->     Function that maps lambda to size

    // Create a safe group of unknown order
    let p = BigNumber::safe_prime(1024);
    let q = BigNumber::safe_prime(1024);

    // Calculate the modulus
    let modulus = p.clone() * q.clone();

    // Create the group
    let group = Group { modulus };

    // // Get the exponent from the lambda
    // let exp = BigNumber::from(lambda);

    // // 2 !!!!!    -----         (Group modulus?)

    // let two = BigNumber::from(2);

    // let a = BigNumber::from(two.clone()).modpow(&exp, &group.modulus);
    // let b = BigNumber::from(two.clone()).modpow(&(exp + 1), &group.modulus);

    // Create the hash function using map_string_to_prime
    let hash_function: Arc<dyn Fn(&str) -> BigNumber + Send + Sync> =
        { Arc::new(move |input: &str| map_string_to_prime(input)) };

    // Generator g
    let g = BigNumber::random(&group.modulus);

    let symb = jacobi(&g, &group.modulus);

    println!("symb: {:?}", symb);

    // Create the public parameters
    let pp = (group, g.clone(), hash_function);
    let c = (BigNumber::from(1), g.clone());

    // Return the public parameters
    (pp, c)
}
