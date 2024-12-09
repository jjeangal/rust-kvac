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
        let key = "test".to_string();
        let value = BigNumber::from(8);

        // Insert a key-value pair
        let (new_c, proof_k, op) = insert(commitment.clone(), (key, value));

        // Verify the proof
        let result = verify(new_c, (op.key, op.value), proof_k);

        // Assert that the verification is successful
        assert!(result, "Verification should succeed with generated data");
    }

    #[test]
    fn test_verify_with_invalid_commitment() {
        let invalid_commitment: Commitment =
            Commitment::new(BigNumber::from(0), BigNumber::from(0)); // Invalid commitment
        let key = "test_key".to_string();
        let value = BigNumber::from(5);

        let proof = Proof::new(
            invalid_commitment.clone(),
            (BigNumber::from(1), BigNumber::from(1), BigNumber::from(1)),
            BigNumber::from(1),
        ); // Dummy proof

        let result = verify(invalid_commitment, (key, value), proof);
        assert!(!result, "Verification should fail with invalid commitment");
    }

    #[test]
    fn test_verify_with_invalid_proof() {
        let valid_commitment: Commitment = Commitment::new(BigNumber::from(1), BigNumber::from(2)); // Example valid commitment
        let key = "test_key".to_string();
        let value = BigNumber::from(5);

        let invalid_proof: Proof = Proof::new(
            Commitment::new(BigNumber::from(0), BigNumber::from(0)),
            (BigNumber::from(0), BigNumber::from(0), BigNumber::from(0)),
            BigNumber::from(0),
        ); // Invalid proof

        let result = verify(valid_commitment, (key, value), invalid_proof);
        assert!(!result, "Verification should fail with invalid proof");
    }

    #[test]
    fn test_verify_with_invalid_key_value_pair() {
        let valid_commitment: Commitment = Commitment::new(BigNumber::from(1), BigNumber::from(2)); // Example valid commitment
        let invalid_key = "".to_string(); // Invalid key (empty)
        let invalid_value = BigNumber::from(-1); // Invalid value (negative)
        let proof = Proof::new(
            valid_commitment.clone(),
            (BigNumber::from(1), BigNumber::from(1), BigNumber::from(1)),
            BigNumber::from(1),
        ); // Example valid proof

        let result = verify(valid_commitment, (invalid_key, invalid_value), proof);
        assert!(
            !result,
            "Verification should fail with invalid key-value pair"
        );
    }
}
