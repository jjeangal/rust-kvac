use crate::params::*;

/// Updates a key-value pair in the commitment.
///
/// # Arguments
///
/// * `commitment` - The current commitment.
/// * `(key, delta)` - A tuple containing the key (as a string) and the update value.
///
/// # Returns
///
/// A tuple containing the new commitment and the update information.
pub fn update(commitment: &Commitment, kv: &KeyValue) -> (Commitment, KeyValue) {
    // Access the public parameters
    let params = &*PUBLIC_PARAMS;
    // Calculate z using the hash function
    let z = (params.hash_function)(kv.key());

    // Calculate C1^z mod modulus
    let c1_z = commitment.c1().modpow(&z, &params.group.modulus);
    // Calculate C2^delta mod modulus
    let c2_delta = commitment.c2().modpow(kv.value(), &params.group.modulus);
    // Calculate C2^z mod modulus
    let c2_z = commitment.c2().modpow(&z, &params.group.modulus);

    // C = (C1^z * C2^delta, C2^z)
    let new_c1 = c1_z.modmul(&c2_delta, &params.group.modulus);

    let new_commitment = Commitment::new(new_c1, c2_z);

    (new_commitment, kv.clone())
}
