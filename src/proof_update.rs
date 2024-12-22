use crate::error::ProofError;
use crate::params::*;
use unknown_order::*;

/// Updates the proof for a given key and update information.
///
/// # Arguments
///
/// * `key` - The key as a string.
/// * `proof` - The current proof.
/// * `kv` - The key-value pair for the update.
///
/// # Returns
///
/// A `Result` containing the updated proof or an error message.
pub fn proof_update(key: &String, proof: &Proof, kv: &KeyValue) -> Result<Proof, ProofError> {
    let params = &*PUBLIC_PARAMS;
    let z = (params.hash_function)(&key);

    match kv.key() == key {
        true => {
            // Case where upd2 = k (same key update)
            // Set Λk = ((Λk,1;(Λk,2)^z),(Λk,3;Λk,4;Λk,5), uk + 1)

            // Calculate (Λk,2)^z
            let k2_z = proof.lambda_k().c2().modpow(&z, &params.group.modulus);

            // Create new first commitment (Λk,1,(Λk,2)^z)
            let new_lambda_k = Commitment::new(proof.lambda_k().c1().clone(), k2_z);

            // Create new proof with updated components
            Ok(Proof::new(
                new_lambda_k.clone(),
                proof.lambda_aux().clone(), // (Λk,3,Λk,4,Λk,5) stays the same
                proof.uk() + BigNumber::one(),
            ))
        }
        false => {
            // Case where upd2 ≠ k (different key update)
            let z_hat = (params.hash_function)(kv.key());

            // Compute α, β ∈ Z such that α · z + β · z_hat = 1
            let GcdResult { gcd, y: beta, .. } = z.extended_gcd(&z_hat);

            if gcd != BigNumber::one() {
                return Err(ProofError::NonCoprimePair);
            }

            // Compute γ = β · Λk,5 mod z
            let gamma = beta.modmul(&proof.lambda_aux().2, &z);

            // Compute η ∈ Z such that γ · z_hat + η · z = Λk,5
            let lhs = proof.lambda_aux().2.modsub(
                &gamma.modmul(&z_hat, &params.group.modulus),
                &params.group.modulus,
            );
            let eta = lhs.moddiv(&z, &params.group.modulus);

            // Compute new components according to the paper
            let new_k1 = proof
                .lambda_k()
                .c1()
                .modpow(&z_hat, &params.group.modulus)
                .modmul(
                    &proof
                        .lambda_k()
                        .c2()
                        .modpow(kv.value(), &params.group.modulus),
                    &params.group.modulus,
                );
            let new_k2 = proof.lambda_k().c2().modpow(&z_hat, &params.group.modulus);
            let new_k3 = proof.lambda_aux().0.modpow(&z_hat, &params.group.modulus);
            let new_k4 = proof.lambda_aux().1.modmul(
                &proof.lambda_aux().0.modpow(&eta, &params.group.modulus),
                &params.group.modulus,
            );

            Ok(Proof::new(
                Commitment::new(new_k1, new_k2),
                (new_k3, new_k4, gamma),
                proof.uk().clone(),
            ))
        }
    }
}

// Add new batch function
pub fn batch_proof_update(
    key: &String,
    initial_proof: &Proof,
    updates: &[KeyValue],
) -> Result<Proof, ProofError> {
    let mut current_proof = initial_proof.clone();
    for kv in updates {
        current_proof = proof_update(key, &current_proof, kv)?;
    }
    Ok(current_proof)
}
