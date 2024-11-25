extern crate rust_kvac;

#[cfg(test)]
mod tests {
    use super::*;
    use rust_kvac::insert::insert;
    use rust_kvac::params::PUBLIC_PARAMS;
    use rust_kvac::verify::verify;
    use unknown_order::BigNumber;

    #[test]
    fn test_verify() {
        // Access the public parameters
        let params = &*PUBLIC_PARAMS;

        // Initial values
        let c = (params.one.clone(), params.g.clone());
        let k = "test".to_string();
        let v = BigNumber::from(8);

        // Insert a key-value pair
        let (new_c, proof_k, upd) = insert(c, (k.clone(), v.clone()));

        // Verify the proof
        let result = verify(new_c, upd, proof_k);

        // Assert that the verification is successful
        assert!(result, "Verification should succeed with generated data");
    }

    #[test]
    fn test_verify_with_invalid_commitment() {
        let invalid_commitment = (BigNumber::from(0), BigNumber::from(0)); // Invalid commitment
        let key = "test_key".to_string();
        let value = BigNumber::from(5);
        let proof = (BigNumber::from(1), BigNumber::from(1)); // Dummy proof

        let result = verify(invalid_commitment, (key.clone(), value.clone()), proof);
        assert!(!result, "Verification should fail with invalid commitment");
    }

    #[test]
    fn test_verify_with_invalid_proof() {
        let valid_commitment = (BigNumber::from(1), BigNumber::from(2)); // Example valid commitment
        let key = "test_key".to_string();
        let value = BigNumber::from(5);
        let invalid_proof = (BigNumber::from(0), BigNumber::from(0)); // Invalid proof

        let result = verify(
            valid_commitment,
            (key.clone(), value.clone()),
            invalid_proof,
        );
        assert!(!result, "Verification should fail with invalid proof");
    }

    #[test]
    fn test_verify_with_invalid_key_value_pair() {
        let valid_commitment = (BigNumber::from(1), BigNumber::from(2)); // Example valid commitment
        let invalid_key = "".to_string(); // Invalid key (empty)
        let invalid_value = BigNumber::from(-1); // Invalid value (negative)
        let proof = (BigNumber::from(1), BigNumber::from(1)); // Example valid proof

        let result = verify(
            valid_commitment,
            (invalid_key.clone(), invalid_value.clone()),
            proof,
        );
        assert!(
            !result,
            "Verification should fail with invalid key-value pair"
        );
    }
}
