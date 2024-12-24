#[cfg(test)]
mod tests {
    use rust_kvac::insert::insert;
    use rust_kvac::params::*;
    use rust_kvac::proof_update::proof_update;
    use rust_kvac::update::update;
    use rust_kvac::verify::verify;
    use unknown_order::BigNumber;

    #[test]
    fn test_proof_update_consistency() {
        // Access the public parameters
        let params = &*PUBLIC_PARAMS;

        // Initial values
        let commitment = Commitment::new(params.one.clone(), params.g.clone());
        let kv: KeyValue = KeyValue::new("test".to_string(), BigNumber::from(8));

        // Insert a key-value pair
        let (_, proof_k, _) = insert(&commitment, &kv);

        // Update the proof
        let new_proof_k = proof_update(&kv.key(), &proof_k, &kv).unwrap();

        // Consistency check: applying the same update should yield the same result
        let new_proof_k_again = proof_update(&kv.key(), &proof_k, &kv).unwrap();
        assert_eq!(
            new_proof_k, new_proof_k_again,
            "Proof update should be consistent"
        );
    }

    #[test]
    fn test_proof_update_validity() {
        let pp = &PUBLIC_PARAMS;

        // Initialize the initial commitment
        let commitment = Commitment::new(pp.one.clone(), pp.g.clone());

        // Define keys and values
        let kv1: KeyValue = KeyValue::new("test".to_string(), BigNumber::from(8));
        let kv2: KeyValue = KeyValue::new("test".to_string(), BigNumber::from(10));
        let kv3: KeyValue = KeyValue::new("test".to_string(), BigNumber::from(18));

        // Insert the first key-value pair
        let (commitment2, proof_k1, _) = insert(&commitment, &kv1);

        // Verify the first insert
        let verify_insert = verify(&commitment2, &kv1, &proof_k1);
        assert!(verify_insert.is_ok(), "Initial insert verification failed");

        // Update the first key-value pair
        let (commitment3, kv2) = update(&commitment2, &kv2);

        // Update the proof
        let proof_upd: Proof = proof_update(&kv1.key(), &proof_k1, &kv2).unwrap();

        // Verify the second insert
        let verify_update = verify(&commitment3, &kv3, &proof_upd);
        assert!(verify_update.is_ok(), "Update verification failed");
    }
}
