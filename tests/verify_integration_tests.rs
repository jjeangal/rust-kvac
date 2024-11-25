extern crate rust_kvac;

#[cfg(test)]
mod tests {
    use super::*;
    use rust_kvac::insert::insert;
    use rust_kvac::params::PUBLIC_PARAMS;
    use rust_kvac::proof_update::proof_update;
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

        // Update the proof
        let new_proof_k = proof_update(proof_k, upd.clone());

        // Verify the proof
        let result = verify(new_c, upd, new_proof_k);

        // Assert that the verification is successful
        assert!(result, "Verification should succeed with generated data");
    }
}
