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
pub fn insert(commitment: Commitment, kv: KeyValue) -> (Commitment, Proof, KeyValue) {
    let params = &*PUBLIC_PARAMS;
    let z = (params.hash_function)(kv.key());
    println!("Insert - z: {:?}", z);

    // First create the proof with original commitment
    // Λk = ((C1, C2),(g, 1, 1), 0)
    let proof = Proof::new(
        commitment.clone(),                                     // Original (C1, C2)
        (params.g.clone(), BigNumber::one(), BigNumber::one()), // (g, 1, 1)
        BigNumber::zero(),                                      // uk = 0
    );
    println!("Insert - initial proof: {:?}", proof);

    // Then calculate new commitment C = (C1^z * C2^v, C2^z)
    let new_c2 = commitment.c2().modpow(&z, &params.group.modulus);
    let new_c1 = commitment.c1().modpow(&z, &params.group.modulus).modmul(
        &commitment.c2().modpow(kv.value(), &params.group.modulus),
        &params.group.modulus,
    );

    let new_commitment = Commitment::new(new_c1, new_c2);
    println!("Insert - new_c1: {:?}", new_commitment.c1());
    println!("Insert - new_c2: {:?}", new_commitment.c2());

    (new_commitment, proof, kv)
}
