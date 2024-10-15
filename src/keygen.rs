use crate::group_hash_function::map_string_to_prime;
use unknown_order::*;

// Key generation algorithm for Key Value Commitments (KVaC)
pub fn keygen(
    lambda: usize,
) -> (
    (
        BigNumber,
        BigNumber,
        Group,
        BigNumber,
        Box<dyn Fn(&str) -> BigNumber>,
    ),
    (BigNumber, BigNumber),
) {
    // 1 !!!!!      -----       (size valid?)

    // Create a safe group of unknown order
    let p = BigNumber::safe_prime(1024);
    let q = BigNumber::safe_prime(1024);

    // Calculate the modulus
    let modulus = p.clone() * q.clone();

    let two = BigNumber::from(2);

    // Create the group
    let group = Group { modulus };

    // Get the exponent from the lambda
    let exp = BigNumber::from(lambda);

    // 2 !!!!!    -----         (Group modulus?  +  2^lambda?)

    let a = BigNumber::from(two.clone()).modpow(&exp, &group.modulus);
    let b = BigNumber::from(two.clone()).modpow(&(exp + 1), &group.modulus);

    // // Set V = [0, a) as a vector of BigNumber
    // let a_bytes = a.to_bytes();
    // let a_int = u64::from_be_bytes(
    //     a_bytes[..8]
    //         .try_into()
    //         .expect("BigNumber to u64 conversion failed"),
    // );
    // let v: Vec<BigNumber> = (0..a_int).map(|x| BigNumber::from(x)).collect();

    // // Set K = {0, 1}*
    // let k = vec![BigNumber::from(0), BigNumber::from(1)];

    // Compute bit-length of b
    let zeta = b.bit_length();

    // Create the hash function using map_string_to_prime
    let hash_function: Box<dyn Fn(&str) -> BigNumber> = {
        let limit = BigNumber::from(zeta + 1);
        let b_clone = b.clone();
        Box::new(move |input: &str| map_string_to_prime(limit.clone(), b_clone.clone(), input))
    };

    // Generator g
    let g = BigNumber::random(&group.modulus);

    // Create the public parameters
    let pp = (a.clone(), b.clone(), group, g.clone(), hash_function);
    let c = (BigNumber::from(1), g.clone());

    // Return the public parameters
    (pp, c)
}
