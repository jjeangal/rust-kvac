use crate::params::*;
use crate::utils::BigNumberExt;
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
pub fn verify(commitment: Commitment, kv: KeyValue, proof: Proof) -> bool {
    let params = &*PUBLIC_PARAMS;
    let z = (params.hash_function)(kv.key());
    println!("z: {:?}", z);

    // First Check: (Λk,2)^z = C2
    let lambda_k2_z = proof.first().c2().modpow(&z, &params.group.modulus);
    println!("lambda_k2_z: {:?}", lambda_k2_z);
    println!("commitment.c2: {:?}", commitment.c2());

    if lambda_k2_z != *commitment.c2() {
        println!("First Check: (Λk,2)^z != C2");
        return false;
    }

    // Second Check: (Λk,1)^(z^(uk+1)) * (Λk,2)^v = C1
    let uk_plus_1 = proof.u_k() + &BigNumber::one();
    println!("uk_plus_1: {:?}", uk_plus_1);

    let z_uk_plus_1 = z.pow(&uk_plus_1);
    println!("z_uk_plus_1: {:?}", z_uk_plus_1);

    // Debug prints for the components
    println!("proof.first().c1(): {:?}", proof.first().c1());
    println!("params.group.modulus: {:?}", params.group.modulus);

    let lambda_k1_z_uk_plus_1 = proof
        .first()
        .c1()
        .modpow(&z_uk_plus_1, &params.group.modulus);

    println!("lambda_k1_z_uk_plus_1: {:?}", lambda_k1_z_uk_plus_1);

    let lambda_k2_v = proof.first().c2().modpow(kv.value(), &params.group.modulus);
    println!("lambda_k2_v: {:?}", lambda_k2_v);
    println!("commitment.c1: {:?}", commitment.c1());

    let result = lambda_k1_z_uk_plus_1.modmul(&lambda_k2_v, &params.group.modulus);
    println!("result of modmul: {:?}", result);

    if result != *commitment.c1() {
        println!("Second Check: (Λk,1)^(z^(uk + 1)) * (Λk,2)^v != C1");
        return false;
    }

    // Third Check
    let lambda_k3_z_uk_plus_1 = proof.second().0.modpow(&z_uk_plus_1, &params.group.modulus);

    if lambda_k3_z_uk_plus_1 != *commitment.c2() {
        println!("Third Check: (Λk,3)^(z^(uk + 1)) != C2");
        return false;
    }

    // Fourth Check
    let lambda_k4_z = proof.second().1.modpow(&z, &params.group.modulus);

    let lambda_k3_lambda_k5 = proof
        .second()
        .0
        .modpow(&proof.second().2, &params.group.modulus);

    if lambda_k4_z.modmul(&lambda_k3_lambda_k5, &params.group.modulus) != params.g {
        println!("Fourth Check: (Λk,4)^z * (Λk,3)^(Λk,5) != g");
        return false;
    }

    true
}
