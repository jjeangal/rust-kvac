use crate::params::*;
use unknown_order::BigNumber;

/// Verifies the proof for a given key-value pair.
///
/// # Arguments
///
/// * `commitment` - The current commitment.
/// * `(key, value)` - A tuple containing the key (as a string) and the value.
/// * `proof` - The proof components.
///
/// # Returns
///
/// `true` if the proof is valid, `false` otherwise.
pub fn verify(commitment: Commitment, (key, value): (String, BigNumber), proof: Proof) -> bool {
    // Access the public parameters
    let params = &*PUBLIC_PARAMS;

    // Calculate z using the hash function
    let z = (params.hash_function)(&key);

    // First Check
    // ----------------------------------------------------------------------------------

    // Calculate (Λk,2)^z mod modulus
    let lambda_k2_z = proof.first().c2().modpow(&z, &params.group.modulus);

    // Verify that (Λk,2)^z = C2
    if lambda_k2_z != *commitment.c2() {
        println!("Verification failed: (Λk,2)^z != C2");
        return false;
    }

    // Second Check
    // ----------------------------------------------------------------------------------

    // Calculate uk + 1
    let uk_plus_1 = BigNumber::modadd(proof.u_k(), &BigNumber::one(), &params.group.modulus);

    // Calculate z^(uk + 1) mod modulus
    let z_uk_plus_1 = z.modpow(&uk_plus_1, &params.group.modulus);

    // Calculate (Λk,1)^(z^(uk + 1)) mod modulus
    let lambda_k1_z_uk_plus_1 = proof
        .first()
        .c1()
        .modpow(&z_uk_plus_1, &params.group.modulus);

    // Calculate (Λk,2)^v mod modulus
    let lambda_k2_v = proof.first().c2().modpow(&value, &params.group.modulus);

    // Verify that (Λk,1)^(z^(uk + 1)) * (Λk,2)^v = C1
    if lambda_k1_z_uk_plus_1.modmul(&lambda_k2_v, &params.group.modulus) != *commitment.c1() {
        println!("Verification failed: (Λk,1)^(z^(uk + 1)) * (Λk,2)^v != C1");
        return false;
    }

    // Third Check
    // ----------------------------------------------------------------------------------

    // Calculate (Λk,3)^(z^(uk + 1)) mod modulus
    let lambda_k3_z_uk_plus_1 = proof.second().0.modpow(&z_uk_plus_1, &params.group.modulus);

    // Verify that (Λk,3)^(z^(uk + 1)) = C2
    if lambda_k3_z_uk_plus_1 != *commitment.c2() {
        println!("Verification failed: (Λk,3)^(z^(uk + 1)) != C2");
        return false;
    }

    // Fourth Check
    // ----------------------------------------------------------------------------------

    // Calculate (Λk,4)^z mod modulus
    let lambda_k4_z = proof.second().1.modpow(&z, &params.group.modulus);

    // Calculate (Λk,3)^(Λk,5) mod modulus
    let lambda_k3_lambda_k5 = proof
        .second()
        .0
        .modpow(&proof.second().2, &params.group.modulus);

    // Verify that (Λk,4)^z * (Λk,3)^(Λk,5) = g
    if lambda_k4_z.modmul(&lambda_k3_lambda_k5, &params.group.modulus) != params.g {
        println!("Verification failed: (Λk,4)^z * (Λk,3)^(Λk,5) != g");
        return false;
    }

    true
}
