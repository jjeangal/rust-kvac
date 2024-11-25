use crate::params::PUBLIC_PARAMS;
use unknown_order::*;

// insert with empty key?

/// Inserts a key-value pair into the commitment.
///
/// # Arguments
///
/// * `c` - A tuple representing the current commitment.
/// * `(k, v)` - A tuple containing the key (as a string) and the value.
///
/// # Returns
///
/// A tuple containing the new commitment, the proof, and the update information.
pub fn insert(
    c: (BigNumber, BigNumber),
    (k, v): (String, BigNumber),
) -> (
    (BigNumber, BigNumber),
    (BigNumber, BigNumber),
    (String, BigNumber),
) {
    let params = &*PUBLIC_PARAMS;

    // Calculate z using the hash function
    let z = (params.hash_function)(&k);

    // Calculate C1^z mod modulus and C2^v mod modulus
    let c1_z = &c.0.modpow(&z, &params.group.modulus);
    let c2_v = &c.1.modpow(&v, &params.group.modulus);
    let c2_z = &c.1.modpow(&z, &params.group.modulus);

    // C = (C1^z * C2^v, C2^z)
    let new_c1 = c1_z.clone().modmul(&c2_v, &params.group.modulus);

    let new_c = (new_c1, c2_z.clone());

    (new_c, c, (k, v))
}
