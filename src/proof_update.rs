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
pub fn proof_update(key: String, proof: Proof, kv: KeyValue) -> Result<Proof, String> {
    let params = &*PUBLIC_PARAMS;
    let z = (params.hash_function)(&key);

    match kv.key() == &key {
        true => {
            // Case where upd2 = k (same key update)
            // Set Λk = ((Λk,1;(Λk,2)^z),(Λk,3;Λk,4;Λk,5), uk + 1)

            // Calculate (Λk,2)^z
            let k2_z = proof.first().c2().modpow(&z, &params.group.modulus);

            // Create new first commitment (Λk,1,(Λk,2)^z)
            let new_first = Commitment::new(proof.first().c1().clone(), k2_z);

            // Increment uk
            let new_uk = proof.u_k() + BigNumber::one();

            // Create new proof with updated components
            let new_proof = Proof::new(
                new_first.clone(),
                proof.second().clone(), // (Λk,3,Λk,4,Λk,5) stays the same
                new_uk.clone(),
            );

            println!("ProofUpdate - before update:");
            println!("proof.first().c1(): {:?}", proof.first().c1());
            println!("proof.first().c2(): {:?}", proof.first().c2());
            println!("proof.u_k(): {:?}", proof.u_k());

            println!("ProofUpdate - after update:");
            println!("new_first.c1(): {:?}", new_first.c1());
            println!("new_first.c2(): {:?}", new_first.c2());
            println!("new_uk: {:?}", new_uk);

            Ok(new_proof)
        }
        false => {
            // Case where upd2 ≠ k (different key update)
            let z_hat = (params.hash_function)(kv.key());

            // Compute α, β ∈ Z such that α · z + β · z_hat = 1
            let GcdResult { gcd, x: _, y: beta } = z.extended_gcd(&z_hat);

            if gcd != BigNumber::one() {
                return Err("GCD is not 1, z and z_hat are not coprime".to_string());
            }

            // Compute γ = β · Λk,5 mod z
            let gamma = beta.modmul(&proof.second().2, &z);

            // Compute η ∈ Z such that γ · z_hat + η · z = Λk,5
            let lhs = proof.second().2.modsub(
                &gamma.modmul(&z_hat, &params.group.modulus),
                &params.group.modulus,
            );
            let eta = lhs.moddiv(&z, &params.group.modulus);

            // Compute new components according to the paper
            let new_k1 = proof
                .first()
                .c1()
                .modpow(&z_hat, &params.group.modulus)
                .modmul(
                    &proof.first().c2().modpow(kv.value(), &params.group.modulus),
                    &params.group.modulus,
                );
            let new_k2 = proof.first().c2().modpow(&z_hat, &params.group.modulus);
            let new_k3 = proof.second().0.modpow(&z_hat, &params.group.modulus);
            let new_k4 = proof.second().1.modmul(
                &proof.second().0.modpow(&eta, &params.group.modulus),
                &params.group.modulus,
            );

            let new_proof = Proof::new(
                Commitment::new(new_k1, new_k2),
                (new_k3, new_k4, gamma),
                proof.u_k().clone(),
            );

            Ok(new_proof)
        }
    }
}
