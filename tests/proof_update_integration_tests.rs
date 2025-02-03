mod common;

#[cfg(test)]
mod tests {
    use super::common::context::TestContext;
    use rust_kvac::{
        insert::insert, params::Proof, proof_update::proof_update, update::update, verify::verify,
    };

    #[test]
    fn test_proof_update_consistency() {
        let ctx = TestContext::setup();

        // Insert a key-value pair
        let (commitment1, proof_k, _) = insert(&ctx.initial_commitment, &ctx.kv1);

        // Verify the first insert
        let verify_insert = verify(&commitment1, &ctx.kv1, &proof_k);
        assert!(verify_insert.is_ok(), "Initial insert verification failed");

        // Update the proof
        let new_proof_k = proof_update(&ctx.kv1.key(), &proof_k, &ctx.kv1).unwrap();

        // Consistency check: applying the same update should yield the same result
        let new_proof_k_again = proof_update(&ctx.kv1.key(), &proof_k, &ctx.kv1).unwrap();
        assert_eq!(
            new_proof_k, new_proof_k_again,
            "Proof update should be consistent"
        );
    }

    #[test]
    fn test_proof_update_validity() {
        let ctx = TestContext::setup();

        // Insert the first key-value pair
        let (commitment1, proof_k1, _) = insert(&ctx.initial_commitment, &ctx.kv1);

        // Verify the first insert
        let verify_insert = verify(&commitment1, &ctx.kv1, &proof_k1);
        assert!(verify_insert.is_ok(), "Initial insert verification failed");

        // Update the first key-value pair
        let (commitment2, _) = update(&commitment1, &ctx.kv2);

        // Update the proof
        let proof_upd: Proof = proof_update(&ctx.kv1.key(), &proof_k1, &ctx.kv2).unwrap();

        // Verify the second insert
        let verify_update = verify(&commitment2, &ctx.kv3, &proof_upd);
        assert!(verify_update.is_ok(), "Update verification failed");
    }

    #[test]
    fn test_proof_update_validity_with_different_key() {
        let ctx = TestContext::setup();

        // Insert the first key-value pair
        let (commitment1, proof_k1, _) = insert(&ctx.initial_commitment, &ctx.kv1);

        // Verify the first insert
        let verify_insert = verify(&commitment1, &ctx.kv1, &proof_k1);
        assert!(verify_insert.is_ok(), "Initial insert verification failed");

        // Insert the second key-value pair
        let (commitment2, proof_k2, _) = insert(&commitment1, &ctx.kv4);

        // Second insert verification on the second key-value pair
        let verify_insert = verify(&commitment2, &ctx.kv5, &proof_k2);
        assert!(
            verify_insert.is_ok(),
            "Insert verification with different key should pass"
        );

        // Update the proof for the first key-value pair
        let proof_upd: Proof = proof_update(&ctx.kv1.key(), &proof_k1, &ctx.kv4).unwrap();

        println!("proof_upd: {:?}", proof_upd);

        // Verify the proof update for the first key-value pair
        let verify_update = verify(&commitment2, &ctx.kv4, &proof_upd);
        assert!(verify_update.is_ok(), "Proof update verification failed");

        // // Update the proof for the second key-value pair
        // let proof_upd: Proof = proof_update(&ctx.kv2.key(), &proof_k2, &ctx.kv1).unwrap();

        // // println!("proof_upd: {:?}", proof_upd);

        // // Verify the proof update for the second key-value pair
        // let verify_update = verify(&commitment2, &ctx.kv3, &proof_upd);
        // assert!(verify_update.is_ok(), "Proof update verification failed");
    }
}
