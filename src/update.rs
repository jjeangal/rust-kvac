use crate::params::*;
use unknown_order::*;

/// Updates a key-value pair in the commitment.
///
/// # Arguments
///
/// * `commitment` - A tuple representing the current commitment.
/// * `(key, value)` - A tuple containing the key (as a string) and the value.
///
/// # Returns
///     
/// A tuple containing the new commitment and the operation information.
pub fn update(
    commitment: Commitment,
    (key, value): (String, BigNumber),
) -> (Commitment, Operation) {
    // Access the public parameters
    let params = &*PUBLIC_PARAMS;

    // Calculate z using the hash function
    let z = (params.hash_function)(&key);

    // Calculate C1^z mod modulus and C2^v mod modulus
    let c1_z = commitment.c1().modpow(&z, &params.group.modulus);
    let c2_v = commitment.c2().modpow(&value, &params.group.modulus);

    // Calculate C2^z mod modulus
    let c2_z = commitment.c2().modpow(&z, &params.group.modulus);

    // C = (C1^z * C2^v, C2^z)
    let new_c1 = c1_z.clone().modmul(&c2_v, &params.group.modulus);
    let new_commitment = Commitment::new(new_c1, c2_z);

    // Create the operation record
    let operation = Operation::new(OperationType::Update, key, value);

    (new_commitment, operation)
}
