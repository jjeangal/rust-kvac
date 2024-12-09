extern crate rust_kvac;

#[cfg(test)]
mod tests {
    use super::*;
    use rust_kvac::insert::insert;
    use rust_kvac::params::*;
    use rust_kvac::proof_update::proof_update;
    use unknown_order::BigNumber;

    #[test]
    fn test_proof_update_consistency() {
        // Access the public parameters
        let params = &*PUBLIC_PARAMS;

        // Initial values
        let commitment = Commitment::new(params.one.clone(), params.g.clone());
        let key = "test".to_string();
        let value = BigNumber::from(8);

        // Insert a key-value pair
        let (_, proof_k, upd) = insert(commitment.clone(), (key.clone(), value.clone()));

        // Update the proof
        let new_proof_k = proof_update(key.clone(), proof_k.clone(), upd.clone());

        // Consistency check: applying the same update should yield the same result
        let new_proof_k_again = proof_update(key.clone(), proof_k.clone(), upd.clone());
        assert_eq!(
            new_proof_k, new_proof_k_again,
            "Proof update should be consistent"
        );
    }
}
