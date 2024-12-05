use crate::params::*;
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
    commmitment: Commitment,
    (key, value): (String, BigNumber),
) -> (Commitment, Proof, Operation) {
    // Access the public parameters
    let params = &*PUBLIC_PARAMS;

    // Calculate z using the hash function
    let z = (params.hash_function)(&key);

    // Calculate C1^z mod modulus and C2^v mod modulus
    let c1_z = commmitment.c1().modpow(&z, &params.group.modulus);
    let c2_v = commmitment.c2().modpow(&value, &params.group.modulus);

    // Calculate C2^z mod modulus
    let c2_z = commmitment.c2().modpow(&z, &params.group.modulus);

    // C = (C1^z * C2^v, C2^z)
    let new_c1 = c1_z.clone().modmul(&c2_v, &params.group.modulus);
    let new_commitment = Commitment::new(new_c1, c2_z);

    // Create the proof
    let new_proof = Proof::new(
        commmitment,
        (params.g.clone(), BigNumber::one(), BigNumber::one()),
        BigNumber::zero(),
    );

    // Create the operation record
    let operation = Operation::new(OperationType::Insert, key, value);

    (new_commitment, new_proof, operation)
}
