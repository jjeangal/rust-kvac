extern crate rust_kvac;

#[cfg(test)]
mod tests {
    use super::*;
    use rust_kvac::insert::insert;
    use rust_kvac::params::*;
    use rust_kvac::verify::verify;
    use unknown_order::BigNumber;

    #[test]
    fn test_verify_insert() {
        // Access the public parameters
        let params = &*PUBLIC_PARAMS;

        // Initial values
        let commitment = Commitment::new(params.one.clone(), params.g.clone());
        let kv = KeyValue::new("test".to_string(), BigNumber::from(8));

        // Insert a key-value pair
        let (new_c, proof_k, op) = insert(&commitment, &kv);

        // Verify the proof
        let result = verify(&new_c, &op, &proof_k);

        // Assert that the verification is successful
        assert!(
            result.is_ok(),
            "Verification should succeed with generated data"
        );
    }

    #[test]
    fn test_verify_with_invalid_commitment() {
        let invalid_commitment: Commitment =
            Commitment::new(BigNumber::from(0), BigNumber::from(0)); // Invalid commitment
        let kv = KeyValue::new("test_key".to_string(), BigNumber::from(5));

        let proof = Proof::new(
            invalid_commitment.clone(),
            (BigNumber::from(1), BigNumber::from(1), BigNumber::from(1)),
            BigNumber::from(1),
        ); // Dummy proof

        let result = verify(&invalid_commitment, &kv, &proof);
        assert!(
            result.is_err(),
            "Verification should fail with invalid commitment"
        );
    }

    #[test]
    fn test_verify_with_invalid_proof() {
        let valid_commitment: Commitment = Commitment::new(BigNumber::from(1), BigNumber::from(2)); // Example valid commitment
        let kv = KeyValue::new("test_key".to_string(), BigNumber::from(5));

        let invalid_proof: Proof = Proof::new(
            Commitment::new(BigNumber::from(0), BigNumber::from(0)),
            (BigNumber::from(0), BigNumber::from(0), BigNumber::from(0)),
            BigNumber::from(0),
        ); // Invalid proof

        let result = verify(&valid_commitment, &kv, &invalid_proof);
        assert!(
            result.is_err(),
            "Verification should fail with invalid proof"
        );
    }

    #[test]
    fn test_verify_with_invalid_key_value_pair() {
        let valid_commitment: Commitment = Commitment::new(BigNumber::from(1), BigNumber::from(2)); // Example valid commitment
        let kv = KeyValue::new("".to_string(), BigNumber::from(-1));

        let proof = Proof::new(
            valid_commitment.clone(),
            (BigNumber::from(1), BigNumber::from(1), BigNumber::from(1)),
            BigNumber::from(1),
        ); // Example valid proof

        let result = verify(&valid_commitment, &kv, &proof);
        assert!(
            result.is_err(),
            "Verification should fail with invalid key-value pair"
        );
    }
}
