use crate::params::*;
use unknown_order::*;

// String not used

/// Updates the proof for a given key and update information.
///
/// # Arguments
///
/// * `proof_k` - A tuple representing the current proof components.
/// * `upd` - A tuple containing the key (as a string) and the update value.
///
/// # Returns
///
/// A tuple containing the updated proof components.
pub fn proof_update(key: String, proof: Proof, update: Operation) -> Result<Proof, String> {
    // Access the public parameters
    let params = &*PUBLIC_PARAMS;

    // Compute the hash of the key
    let z = (params.hash_function)(&key);

    match update.key == key {
        true => {
            // Calculate C2^z mod modulus
            let k2_z = proof.first().c2().modpow(&z, &params.group.modulus);

            // Create the new first commitment
            let new_first = Commitment::new(proof.first().c1().clone(), k2_z);

            // Increment uk
            let new_uk = BigNumber::modadd(proof.u_k(), &BigNumber::one(), &params.group.modulus);

            // Create the new proof
            let new_proof = Proof::new(new_first, proof.second().clone(), new_uk);

            // Return the updated proof
            Ok(new_proof)
        }
        false => {
            // Compute the hash of the update key
            let z_hat = (params.hash_function)(&update.key);

            // Compute α, β ∈ Z such that α · z + β · z_hat = 1
            let GcdResult { gcd, x: _, y } = &z.extended_gcd(&z_hat);

            if *gcd != BigNumber::one() {
                return Err("GCD is not 1, z and z_hat are not coprime".to_string());
            }

            // Compute γ = β · Λk,5 mod z
            let gamma = y.modmul(&proof.second().2, &z);

            // Compute η ∈ Z such that γ · z_hat + η · z = Λk,5
            // Do so by computing Λk,5 - γ · z_hat
            let lhs = proof.second().2.modsub(
                &gamma.modmul(&z_hat, &params.group.modulus),
                &params.group.modulus,
            );

            // Compute η = lhs / z
            let eta = lhs.moddiv(&z, &params.group.modulus);

            // Compute the new first commitment
            // C1^z_hat · C2^v mod modulus
            let k1_z_hat = proof.first().c1().modpow(&z_hat, &params.group.modulus);
            let k2_z_upd_val = proof
                .first()
                .c2()
                .modpow(&update.value, &params.group.modulus);

            // Compute the new k1 = C1^z_hat · C2^v
            let new_k1 = k1_z_hat.modmul(&k2_z_upd_val, &params.group.modulus);

            // Compute the new k2 = C2^z_hat
            let new_k2 = proof.first().c2().modpow(&z_hat, &params.group.modulus);

            // C = (C1^z_hat · C2^v, C2^z_hat)
            let proof_first = Commitment::new(new_k1, new_k2);

            // Compute the new k3 = k3^z_hat
            let new_k3 = proof.second().0.modpow(&z_hat, &params.group.modulus);

            // Computer the new k4 = k4 · k3^η
            let k3_eta = proof.second().0.modpow(&eta, &params.group.modulus);
            let new_k4 = proof.second().1.modmul(&k3_eta, &params.group.modulus);

            // Create the new proof
            let new_proof = Proof::new(proof_first, (new_k3, new_k4, gamma), proof.u_k().clone());

            Ok(new_proof)
        }
    }
}
